use serde::{Deserialize, Serialize};
use candid::Principal;
use dfn_protobuf::{ProtoBuf, ToProto};
use ic_types::CanisterId;
use ic_agent::Agent;
use on_wire::{FromWire, IntoWire};

pub const LOGFIX: &str = "================================================================================================";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Error {
    MalformedMessage(String),
    KeyNotPresent(Vec<u8>),
    KeyAlreadyPresent(Vec<u8>),
    VersionNotLatest(Vec<u8>),
    VersionBeyondLatest(Vec<u8>),
    RegistryUnreachable(String),
    UnknownError(String),
}

pub async fn query<Payload: ToProto, Res: ToProto>(
    agent: &Agent,
    canister_id: Principal,
    method: &str, 
    payload: Payload,
) -> Result<Res, String> {
    let arg = ProtoBuf(payload).into_bytes()?;
    let bytes = agent
        .query(&canister_id, method)
        .with_arg(arg)
        .call()
        .await
        .expect("query pb interface error.");
    ProtoBuf::from_bytes(bytes).map(|c| c.0)
}