use ic_registry_keys::{make_icp_xdr_conversion_rate_record_key, make_subnet_list_record_key,
    make_unassigned_nodes_config_record_key, make_replica_version_key,
    make_blessed_replica_version_key, make_routing_table_record_key,
    make_firewall_config_record_key, make_provisional_whitelist_record_key,
    make_node_operator_record_key, make_crypto_tls_cert_key, make_node_record_key,
    make_data_center_record_key, make_crypto_threshold_signing_pubkey_key,
    make_catch_up_package_contents_key, make_subnet_record_key, make_nns_canister_records_key
    };
use ic_registry_transport::pb::v1::{RegistryGetValueRequest, RegistryGetValueResponse};
use ic_protobuf::registry::{
    subnet::v1::{SubnetListRecord, SubnetRecord, CatchUpPackageContents}, routing_table::v1::RoutingTable, 
    unassigned_nodes_config::v1::UnassignedNodesConfigRecord,
    replica_version::v1::{BlessedReplicaVersions, ReplicaVersionRecord},
    firewall::v1::FirewallConfig, provisional_whitelist::v1::ProvisionalWhitelist,
    node_operator::v1::NodeOperatorRecord,crypto::v1::{X509PublicKeyCert, PublicKey},
    node::v1::NodeRecord, dc::v1::DataCenterRecord,
    nns::v1::NnsCanisterRecords,
};
use ic_nns_constants::{REGISTRY_CANISTER_ID};
use ic_nns_common::registry::get_icp_xdr_conversion_rate_record;
use ic_base_types::{PrincipalId, SubnetId, NodeId};
use ic_agent::Agent;
use candid::Principal;
use prost::Message;
use std::convert::{TryInto, TryFrom};
use std::str::FromStr;

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

pub async fn subnet_list_record(agent: &Agent) {
    let response = get_value(agent, make_subnet_list_record_key()).await
        .expect("routing_table_record error");
    let value = SubnetListRecord::decode(&response.value[..]).expect("Decode subnet list value error.");

    let v: Vec<SubnetId> = value
        .subnets
        .iter()
        .map(|subnet_id_vec| SubnetId::new(PrincipalId::try_from(subnet_id_vec).unwrap()))
        .collect();
    println!("{:?}\n{}", v, v.len());
}

pub async fn unassigned_nodes_config_record(agent: &Agent) {
    let response = get_value(agent, make_unassigned_nodes_config_record_key()).await
    .expect("routing_table_record error");

    let value = UnassignedNodesConfigRecord::decode(&response.value[..]).expect("Decode unassigned nodes config error");
    println!("{:?}", value);
}

pub async fn replica_version(agent: &Agent) {
    let response = get_value(agent, make_replica_version_key("875b404679d46475b705d3575e8f952ed3d43e2f")).await
    .expect("routing_table_record error");
    
    let value = ReplicaVersionRecord::decode(&response.value[..]).expect("Decode unassigned nodes config error");
    println!("{:?}", value);
}

pub async fn blessed_replica_version(agent: &Agent) {
    let response = get_value(agent, make_blessed_replica_version_key()).await
    .expect("routing_table_record error");
    
    let value = BlessedReplicaVersions::decode(&response.value[..]).expect("Decode unassigned nodes config error");
    println!("{:?}", value);
}

pub async fn firewall_config_record(agent: &Agent) {
    let response = get_value(agent, make_firewall_config_record_key()).await
    .expect("routing_table_record error");
    
    let value = FirewallConfig::decode(&response.value[..]).expect("Decode unassigned nodes config error");
    println!("{:?}", value);
}

pub async fn provisional_whitelist_record(agent: &Agent) {
    let response = get_value(agent, make_provisional_whitelist_record_key()).await
    .expect("routing_table_record error");

    let value = ProvisionalWhitelist::decode(&response.value[..]).expect("Decode unassigned nodes config error");
    println!("{:?}", value);
}

pub async fn node_operator_record(agent: &Agent) {
    let response = get_value(agent, make_node_operator_record_key(
        PrincipalId::from(Principal::from_text(
            "qffmn-uqkl2-uuw6l-jo5i6-obdek-tix6f-u4odv-j3265-pcpcn-jy5le-lae"
        ).unwrap()))).await
    .expect("routing_table_record error");

    let value = NodeOperatorRecord::decode(&response.value[..]).expect("Decode unassigned nodes config error");
    println!("{:?}", value);
}

pub async fn crypto_tls_cert(agent: &Agent) {
    let response = get_value(agent, make_crypto_tls_cert_key(
        NodeId::from(PrincipalId::from(Principal::from_text(
                    "5o4ne-ipouv-i46r7-xrjkk-vpyru-xdtq7-redrd-eqa3q-ebvwz-tbbx2-aae"
                ).unwrap())
            )
        )).await
        .expect("routing_table_record error");

    let value = X509PublicKeyCert::decode(&response.value[..]).expect("Decode unassigned nodes config error");
    println!("{:?}", value);
}

pub async fn node_record(agent: &Agent) {
    let response = get_value(agent, make_node_record_key(
        NodeId::from(PrincipalId::from(Principal::from_text(
                    "5o4ne-ipouv-i46r7-xrjkk-vpyru-xdtq7-redrd-eqa3q-ebvwz-tbbx2-aae"
                ).unwrap())
            )
        )).await
        .expect("routing_table_record error");

    let value = NodeRecord::decode(&response.value[..]).expect("Decode unassigned nodes config error");
    println!("{:?}", value);
    let node_operator = Principal::from_slice(&value.node_operator_id);
    println!("{:?}", node_operator.to_text());
}

pub async fn data_center_record(agent: &Agent) {
    let response = get_value(agent, make_data_center_record_key("AN1")).await
    .expect("routing_table_record error");

    let value = DataCenterRecord::decode(&response.value[..]).expect("Decode unassigned nodes config error");
    println!("{:?}", value);
}

pub async fn crypto_threshold_signing_pubkey(agent: &Agent) {
    let response = get_value(agent, make_crypto_threshold_signing_pubkey_key(
        SubnetId::new(PrincipalId::from_str(
            "tdb26-jop6k-aogll-7ltgs-eruif-6kk7m-qpktf-gdiqx-mxtrf-vb5e6-eqe"
        ).unwrap()))).await
    .expect("routing_table_record error");

    let value = PublicKey::decode(&response.value[..]).expect("Decode unassigned nodes config error");
    println!("{:?}", value);
}

pub async fn catch_up_package_contents(agent: &Agent) {
    let response = get_value(agent, make_catch_up_package_contents_key(
        SubnetId::new(PrincipalId::from_str(
            "tdb26-jop6k-aogll-7ltgs-eruif-6kk7m-qpktf-gdiqx-mxtrf-vb5e6-eqe"
        ).unwrap()))).await
    .expect("routing_table_record error");

    let value = CatchUpPackageContents::decode(&response.value[..]).expect("Decode unassigned nodes config error");
    let low = value.initial_ni_dkg_transcript_low_threshold.unwrap();
    let high = value.initial_ni_dkg_transcript_high_threshold.unwrap();
    println!("{:?}\n", low.threshold);
}

pub async fn subnet_record(agent: &Agent) {
    let response = get_value(agent, make_subnet_record_key(
        SubnetId::new(PrincipalId::from_str(
            "tdb26-jop6k-aogll-7ltgs-eruif-6kk7m-qpktf-gdiqx-mxtrf-vb5e6-eqe"
        ).unwrap()))).await
    .expect("routing_table_record error");

    let value = SubnetRecord::decode(&response.value[..]).expect("Decode unassigned nodes config error");
    let membership: Vec<String> = value
        .membership
        .iter()
        .map(|v| Principal::from_slice(v).to_text())
        .collect();
    
    println!("membership: {:?}\nmax_ingress_bytes_per_message: {:?}\n
    unit_delay_millis: {:?}\ninitial_notary_delay_millis: {:?}\nreplica_version_id: {:?}\n
    dkg_interval_length: {}\ngossip_config: {:?}\nstart_as_nns: {}\nsubnet_type: {}\ndkg_dealings_per_block: {}\n
    is_halted: {}\nmax_ingress_messages_per_block: {}\nmax_block_payload_size: {}\nmax_instructions_per_message: {}\n
    max_instructions_per_round: {}\nmax_instructions_per_install_code: {}\nfeatures: {:?}\nmax_number_of_canisters: {}\n
    ssh_readonly_access: {:?}\nssh_backup_access: {:?}\necdsa_config: {:?}", 
    membership.len(), value.max_ingress_bytes_per_message,
    value.unit_delay_millis, value.initial_notary_delay_millis, value.replica_version_id, 
    value.dkg_interval_length, value.gossip_config, value.start_as_nns, value.subnet_type, value.dkg_dealings_per_block, 
    value.is_halted, value.max_ingress_messages_per_block, value.max_block_payload_size, value.max_instructions_per_message, 
    value.max_instructions_per_round, value.max_instructions_per_install_code, value.features, value.max_number_of_canisters, 
    value.ssh_readonly_access, value.ssh_backup_access, value.ecdsa_config);
}

pub async fn nns_canister_records(agent: &Agent) {
    let response = get_value(agent, make_nns_canister_records_key()).await
    .expect("routing_table_record error");

    let value = NnsCanisterRecords::decode(&response.value[..]).expect("Decode unassigned nodes config error");

    println!("{:?}\n", value);
}


