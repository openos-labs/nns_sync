use ic_registry_keys::{make_icp_xdr_conversion_rate_record_key, make_routing_table_record_key};
use ic_registry_transport::pb::v1::{RegistryGetValueRequest, RegistryGetValueResponse};
use ic_protobuf::registry::routing_table::v1::RoutingTable;
use ic_nns_constants::{REGISTRY_CANISTER_ID};
use ic_agent::{Agent};
use candid::Principal;
use prost::Message;
use std::convert::TryInto;

use crate::types::{Error, LOGFIX};

async fn get_value(agent: &Agent, key: String) -> Result<RegistryGetValueResponse, Error> {
    let request_key = key.as_bytes().to_vec();
    let request = RegistryGetValueRequest {
        version: None,
        key: request_key,
    };
    let mut request_buf = Vec::new();
    request.encode(&mut request_buf).expect("Request encode error.");

    let response_bytes = agent.query(
        &REGISTRY_CANISTER_ID.get().0,
        "get_value",
    )
    .with_arg(request_buf)
    .call()
    .await
    .expect("Failed to call canister.");

    match RegistryGetValueResponse::decode(&response_bytes[..]) {
        Ok(result) => Ok(result),
        Err(e) => Err(Error::MalformedMessage(e.to_string())),
    }
}

pub async fn icp_xdr_conversion_rate_record(agent: &Agent) {
    let response = get_value(agent, make_icp_xdr_conversion_rate_record_key()).await
    .expect("icp_xdr_conversion_rate_record error");
    println!("icp_xdr_conversion_rate_record:\n{:?}", response);
}

pub async fn routing_table_record(agent: &Agent) {
    let response = get_value(agent, make_routing_table_record_key()).await
    .expect("routing_table_record error");
    let value = RoutingTable::decode(&response.value[..]).expect("Decode routing table value error.");
    println!("routing_table_record:\n{}\nerror: {:?} version: {}\n", LOGFIX, response.error, response.version);
    for v in value.entries.iter() {
        let subnet_id = Principal::from_slice(&v.subnet_id.as_ref().unwrap().principal_id.as_ref().unwrap().raw);
        println!("subnet id: {:?}", subnet_id.to_text());

        let canister_ranges = v.range.as_ref().unwrap();
        let start = canister_ranges.start_canister_id.as_ref().unwrap().principal_id.as_ref().unwrap();
        let end = canister_ranges.end_canister_id.as_ref().unwrap().principal_id.as_ref().unwrap();
        let start_num = u64::from_be_bytes(start.raw[..8].try_into().unwrap());
        let end_num = u64::from_be_bytes(end.raw[..8].try_into().unwrap());
        let start_prin = Principal::from_slice(&start.raw);
        let end_prin = Principal::from_slice(&end.raw);

        println!("start PrincipalId: {:?}\nstart PrincipalId in Text: {}\nstart PrincipalId from u64: {}", start, start_prin.to_text(), start_num);
        println!("end PrincipalId: {:?}\nend PrincipalId in Text: {}\nend PrincipalId from u64: {}\n", end, end_prin.to_text(), end_num);
    }
}
