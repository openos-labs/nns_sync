use ic_agent::{
    agent::http_transport::ReqwestHttpReplicaV2Transport, ic_types::Principal,
    identity::AnonymousIdentity, Agent,
};
use ic_protobuf::registry::routing_table::v1::RoutingTable;
use ic_registry_transport::pb::v1::{RegistryGetValueResponse, RegistryGetValueRequest};
use prost::Message;

mod types;
mod registry;

const DEFAULT_IC_GATEWAY: &str = "https://ic0.app";

#[tokio::main]
async fn main() {
    let agent = Agent::builder()
        .with_transport(
            ReqwestHttpReplicaV2Transport::create(DEFAULT_IC_GATEWAY)
                .expect("Failed to create Transport for Agent"),
        )
        .with_identity(AnonymousIdentity{})
        .build()
        .expect("Failed to build the Agent");
    // registry::icp_xdr_conversion_rate_record(&agent).await;
    registry::routing_table_record(&agent).await;
}


