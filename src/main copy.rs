use candid::{types::number::Nat, Decode, Encode};
use ic_agent::{
    agent::http_transport::ReqwestHttpReplicaV2Transport, ic_types::Principal,
    identity::AnonymousIdentity, Agent,
};
use serde::Deserialize;
use ic_protobuf::registry::routing_table::v1::RoutingTable;
use std::{fs, thread, time::Duration};


const CONFIG_PATH: &str = "./config.json";
const DEFAULT_IC_GATEWAY: &str = "https://ic0.app";
const INTERVAL: u64 = 1 * 60 * 1000; // 1 min
const INTERVAL_S: u64 = 0; // 0 s
const QUERY_NUM: usize = 100;
const QUERY_NUM_S: usize = 13000;

#[derive(Debug, Deserialize)]
struct CanisterConfig {
    canister_id: String,
    canister_type: String,
}

#[tokio::main]
async fn main() {
    let config_data = fs::read_to_string(CONFIG_PATH).expect("Unable to read config file");
    let config_list: Vec<CanisterConfig> = serde_json::from_str(config_data.as_str()).unwrap();

    let agent = Agent::builder()
        .with_transport(
            ReqwestHttpReplicaV2Transport::create(DEFAULT_IC_GATEWAY)
                .expect("Failed to create Transport for Agent"),
        )
        .with_identity(AnonymousIdentity{})
        .build()
        .expect("Failed to build the Agent");

    let mut join_handle_list = vec![];
    for config in config_list {
        println!("Starting following canister: {:?}", config);
        let handle = tokio::spawn(async move {
            sync_canister(config, &agent).await;
        });
        join_handle_list.push(handle);
    }
    for handle in join_handle_list {
        handle.await.unwrap();
    }
}

pub async fn sync_canister(config: CanisterConfig, agent: &Agent) {
    match config.canister_type {
        String::from("registry") => {
            let request_bytes = "routing_table".to_string().as_bytes().to_vec();
            let response = agent
            .query(
                &Principal::from_text(config.canister_id.clone()).expect(
                    format!(
                        "Failed to convert this canister_id to principal: {}",
                        canister_id
                    )
                    .as_str(),
                ),
                "get_value",
            )
            .with_arg(&Encode!(&request_bytes).unwrap())
            .call()
            .await
            .expect("Failed to call canister on historySize.");
            let result = Decode!(response.as_slice(), Vec<u8>)
                .expect("Failed to decode the getTransactions response data.");
        },
        _ => {}
    }
}
