use ic_agent::{
    agent::http_transport::ReqwestHttpReplicaV2Transport, ic_types::Principal,
    identity::AnonymousIdentity, Agent,
};
use ic_protobuf::registry::routing_table::v1::RoutingTable;
use ic_registry_transport::pb::v1::{RegistryGetValueRequest, RegistryGetValueResponse};
use ledger_canister::EncodedBlock;
use ledger_canister::{Block, Operation};
use mysql::{Opts, Pool};
use on_wire::NewType;
use prost::Message;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use std::cmp::min;
use std::env;
use types::Transaction;
mod ledger;
mod registry;
mod types;
use crate::ledger::{block_pb, tip_of_chain_pb};

use mysql::prelude::*;
use mysql::*;
use std::sync::Arc;
use std::sync::RwLock;
use std::{thread, time};
use tokio::task::JoinHandle;
const DEFAULT_IC_GATEWAY: &str = "https://ic0.app";

pub async fn insert_to_mysql(data: Vec<Transaction>, conn: &Rbatis) -> u8 {
    //     let stmt = conn.prep("INSERT INTO transactions_test (id, hash, blockhash, type_, createdtime, from_, to_, amount, fee, memo) VALUES (:id, :hash, :blockhash, :type_, :createdtime, :from_, :to_, :amount, :fee, :memo)")
    //  .unwrap();
    //     for i in 0..data.len() {
    //         conn.exec_drop(
    //             &stmt,
    //             params! {
    //                 "id" => data[i].id,
    //                 "hash" => data[i].hash.clone(),
    //                 "blockhash" => data[i].blockhash.clone(),
    //                 "type_" => data[i].type_.clone(),
    //                 "createdtime" => data[i].createdtime,
    //                 "from_" => data[i].from.clone(),
    //                 "to_" => data[i].to.clone(),
    //                 "amount" => data[i].amount,
    //                 "fee" => data[i].fee,
    //                 "memo" => data[i].memo.clone(),
    //             },
    //         )
    //         .unwrap()
    //     }
    let result = conn.save_batch(&data, &[]).await;
    if let Ok(_) = result {
        1
    } else {
        println!("insert to mysql error, {:?}, retrying...", result);
        0
    }
    //println!("last generated key: {}")
}
pub fn get_block_height(conn: &mut PooledConn) -> u64 {
    let res: Result<Option<u64>> = conn.query_first("select count(1) from transactions");
    return res.unwrap().unwrap();
}
pub fn convert_to_mysqldata(block: Block, id: u64) -> Transaction {
    let mut transaction = Transaction {
        id: id,
        hash: hex::encode(block.transaction.hash().into_bytes()),
        blockhash: String::from(""),
        type_: String::from(""),
        createdtime: block.transaction.created_at_time.timestamp_nanos,
        from_: String::from(""),
        to_: String::from(""),
        amount: 0,
        fee: 0,
        memo: block.transaction.memo.0.to_string(),
    };
    match block.transaction.operation {
        Operation::Mint {
            to: to_,
            amount: amount_,
        } => {
            transaction.amount = amount_.get_e8s();
            transaction.to_ = to_.to_hex();
            transaction.type_ = String::from("Mint");
        }
        Operation::Burn {
            from: from_,
            amount: amount_,
        } => {
            transaction.amount = amount_.get_e8s();
            transaction.from_ = from_.to_hex();
            transaction.type_ = String::from("Burn");
        }
        Operation::Transfer {
            from: from_,
            to: to_,
            fee: fee_,
            amount: amount_,
        } => {
            transaction.amount = amount_.get_e8s();
            transaction.from_ = from_.to_hex();
            transaction.to_ = to_.to_hex();
            transaction.fee = fee_.get_e8s();
            transaction.type_ = String::from("Transfer");
        }
    }
    return transaction;
}

pub async fn get_new_transaction(id: u64, agent: Agent, set: Arc<RwLock<Vec<Transaction>>>) {
    let data = block_pb(&agent, id).await;
    if let Ok(block) = data {
        let transaction = convert_to_mysqldata(block, id + 1);
        let mut s = set.write().unwrap();
        s.push(transaction);
        drop(s);
    }
}

#[tokio::main]
async fn main() {
    let agent = Agent::builder()
        .with_transport(
            ReqwestHttpReplicaV2Transport::create(DEFAULT_IC_GATEWAY)
                .expect("Failed to create Transport for Agent"),
        )
        .with_identity(AnonymousIdentity {})
        .build()
        .expect("Failed to build the Agent");
    let max_thread = 20;
    // let url = "root:xyz12345@(localhost:3306)/xyz";
    let url = "mysql://admin:Gbs1767359487@database-mysql-instance-1.ccggmi9astti.us-east-1.rds.amazonaws.com:3306/db1";
    let rb = Rbatis::new();
    rb.link(url).await.unwrap();
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut height = get_block_height(&mut pool.get_conn().unwrap());
    println!("present sync blocks in database {:?}", height);
    while true {
        let b = ledger::get_blocks_pb(&agent, height, 1000).await;
        if let Some(Blocks) = b {
            let mut new_transactions: Vec<Transaction> = Vec::new();
            for block in Blocks {
                new_transactions.push(convert_to_mysqldata(block, height + 1));
                height += 1;
            }
            insert_to_mysql(new_transactions, &rb).await;
        } else {
            println!("batch sync finish....convert to multi-thread sync");
            break;
            // let ten_seconds = time::Duration::from_secs(10);
            // thread::sleep(ten_seconds);
        }
        println!("==>>sync batch transactions to {:?}", height);
    }

    while true {
        let mut current_height = 0;
        if let Ok(h) = tip_of_chain_pb(&agent).await {
            current_height = h.tip_index + 1;
        } else {
            continue;
        }
        println!("current blocks on IC {:?}", current_height);
        while (height < current_height) {
            let set: Vec<Transaction> = Vec::new();
            let set_arc = Arc::new(RwLock::new(set));
            let mut thread_vec: Vec<JoinHandle<()>> = Vec::new();
            let num_thread = min(max_thread, current_height - height);
            for i in 0..num_thread {
                let agent_clone = agent.clone();
                let set_ = set_arc.clone();
                let handle = tokio::spawn(async move {
                    get_new_transaction(height + i, agent_clone, set_).await
                });
                thread_vec.push(handle);
            }
            for handle in thread_vec {
                handle.await;
            }
            let mut data = (*(set_arc.read().unwrap())).clone();
            let l = data.len();
            if l as u64 != num_thread {
                println!("thread error...num not match, pre heigh {:?}", height);
                continue;
            }

            let result = insert_to_mysql(data, &rb).await;
            if result == 0 {
                continue;
            }
            height = height + l as u64;
            println!("sync to {:?} blocks...", height);
        }
    }
    //println!("{:?}", b);
}
