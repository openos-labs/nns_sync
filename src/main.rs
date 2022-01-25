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
   
    // registry::routing_table_record(&agent).await;
    
    // 获取所有的 subnet id。30 个。
    // registry::subnet_list_record(&agent).await;
    
    // 获取 unassigned 节点的 ssh public key
    // registry::unassigned_nodes_config_record(&agent).await;
    
    // 节点版本详情，节点获取之后下载对应的安装包。需要传入特定的 replica verison id
    // registry::replica_version(&agent).await;

    // 节点版本 id 集合，返回一个给上面⬆用的 id。
    // registry::blessed_replica_version(&agent).await;

    // 防火墙配置
    // registry::firewall_config_record(&agent).await;

    // 零时白名单，用于创世纪，现在已经清空。
    // registry::provisional_whitelist_record(&agent).await;

    // 获取 node operator 信息，包括数据中心 id，operator id，provide id，rewardable_node，需要传入 operator id
    // registry::node_operator_record(&agent).await;
    // 获取某个节点的 X509 public key Cert，需要传入节点 id
    // registry::crypto_tls_cert(&agent).await;
    
    // 获取某个节点的 xnet，http，p2p，prometheus，api，node_operator信息，需要传入节点 id
    // registry::node_record(&agent).await;

    // 获取某个数据中心数据
    // registry::data_center_record(&agent).await;
    
    // 获取某个子网的 bls 门限公钥
    // registry::crypto_threshold_signing_pubkey(&agent).await;

    // 获取 DKG transcript 初始值
    // registry::catch_up_package_contents(&agent).await;

    // 获取子网的成员（节点），以及一些其他的
    registry::subnet_record(&agent).await;


}


