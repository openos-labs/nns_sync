use crate::types::{query, Error, LOGFIX};
use candid::Principal;
use ic_agent::Agent;
use ic_nns_constants::LEDGER_CANISTER_ID;
use ledger_canister::{
    Block, BlockHeight, BlockRes, EncodedBlock, GetBlocksArgs, GetBlocksRes, TipOfChainRes,
};

pub async fn block_pb(agent: &Agent, height: BlockHeight) -> Result<Block, String> {
    let response: Result<BlockRes, String>;
    let encode_block;
    // 至少在 2597000 块高之前的交易记录，都存在了 qjdve-lqaaa-aaaaa-aaaeq-cai。
    if height < 2597000 {
        response = query(
            agent,
            Principal::from_text("qjdve-lqaaa-aaaaa-aaaeq-cai")
                .expect("principal from archive canister err."),
            "get_block_pb",
            height,
        )
        .await;
        encode_block = match response {
            Ok(BlockRes(res)) => match res {
                Some(result_encode_block) => match result_encode_block {
                    Ok(encode_block) => encode_block,
                    Err(e) => return Err(format!("result_encode_block error again {}", e)),
                },
                None => return Err(format!("res none error again")),
            },
            Err(e) => return Err(format!("response error again {}", e)),
        }
    } else {
        let response: Result<BlockRes, String> =
            query(agent, LEDGER_CANISTER_ID.get().0, "block_pb", height).await;
        encode_block = match response {
            Ok(BlockRes(res)) => match res {
                Some(result_encode_block) => match result_encode_block {
                    Ok(encode_block) => encode_block,
                    Err(e) => {
                        println!("data has been transferred to archive node...");
                        let response: Result<BlockRes, String> =
                            query(agent, e.get().0, "get_block_pb", height).await;
                        match response {
                            Ok(BlockRes(res)) => match res {
                                Some(result_encode_block) => match result_encode_block {
                                    Ok(encode_block) => encode_block,
                                    Err(e) => {
                                        return Err(format!(
                                            "result_encode_block error again {}",
                                            e
                                        ))
                                    }
                                },
                                None => return Err(format!("res none error again")),
                            },
                            Err(e) => return Err(format!("response error again {}", e)),
                        }
                    }
                },
                None => return Err(format!("res none error")),
            },
            Err(e) => return Err(format!("response error {}", e)),
        };
    }
    let block = match encode_block.decode() {
        Ok(block) => block,
        Err(e) => return Err(format!("decode error {}", e)),
    };
    Ok(block)
}

pub async fn get_blocks_pb(agent: &Agent, start: u64, length: usize) -> Option<Vec<Block>> {
    let args = GetBlocksArgs { start, length };
    let response: Result<GetBlocksRes, String> = query(
        agent,
        Principal::from_text("qjdve-lqaaa-aaaaa-aaaeq-cai")
            .expect("principal from archive canister err."),
        "get_blocks_pb",
        args,
    )
    .await;
    let encode_blocks_result = response.expect("get_blocks_pb call error.").0;
    if let Ok(encode_blocks) = encode_blocks_result {
        Some(
            encode_blocks
                .into_iter()
                .map(|b| b.decode().unwrap())
                .collect(),
        )
    } else {
        None
    }
}

pub async fn tip_of_chain_pb(agent: &Agent) -> Result<TipOfChainRes, String> {
    let response: Result<TipOfChainRes, String> =
        query(agent, LEDGER_CANISTER_ID.get().0, "tip_of_chain_pb", ()).await;
    if let Ok(result) = response {
        Ok(result)
    } else {
        Err("get height error".to_string())
    }
}
