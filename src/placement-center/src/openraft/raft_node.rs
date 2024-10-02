use openraft::Config;
use std::fmt::Display;
use std::path::Path;
use std::sync::Arc;

use super::store::new_storage;
use super::typeconfig::TypeConfig;
pub type NodeId = u64;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
pub struct Node {
    pub rpc_addr: String,
    pub api_addr: String,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node {{ rpc_addr: {}, api_addr: {} }}",
            self.rpc_addr, self.api_addr
        )
    }
}

pub mod typ {
    use openraft::error::Infallible;

    use crate::openraft::typeconfig::TypeConfig;

    pub type Entry = openraft::Entry<TypeConfig>;

    pub type RaftError<E = Infallible> = openraft::error::RaftError<TypeConfig, E>;
    pub type RPCError<E = Infallible> = openraft::error::RPCError<TypeConfig, RaftError<E>>;

    pub type ClientWriteError = openraft::error::ClientWriteError<TypeConfig>;
    pub type CheckIsLeaderError = openraft::error::CheckIsLeaderError<TypeConfig>;
    pub type ForwardToLeader = openraft::error::ForwardToLeader<TypeConfig>;
    pub type InitializeError = openraft::error::InitializeError<TypeConfig>;

    pub type ClientWriteResponse = openraft::raft::ClientWriteResponse<TypeConfig>;
}

pub type ExampleRaft = openraft::Raft<TypeConfig>;

pub struct RaftNode {}

impl RaftNode {
    pub fn new() -> Self {
        return RaftNode {};
    }

    pub async fn build_node() {
        let config = Config {
            heartbeat_interval: 250,
            election_timeout_min: 299,
            ..Default::default()
        };

        let config = Arc::new(config.validate().unwrap());
        let dir = Path::new("/tmp");
        let (log_store, state_machine_store) = new_storage(&dir).await;
        let kvs = state_machine_store.data.kvs.clone();
    }
}
