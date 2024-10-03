use clients::poll::ClientPool;
use openraft::{Config, Raft};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::path::Path;
use std::sync::Arc;

use super::network::network::Network;
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

pub async fn start_openraft_node(raft_node: Raft<TypeConfig>) {
    let mut nodes = BTreeMap::new();
    let node = Node {
        api_addr: "127.0.0.1:9901".to_string(),
        rpc_addr: "127.0.0.1:9902".to_string(),
    };

    nodes.insert(1, node);
    let _ = raft_node.initialize(nodes).await;
}

pub async fn create_raft_node(client_poll: Arc<ClientPool>) -> Raft<TypeConfig> {
    let config = Config {
        heartbeat_interval: 250,
        election_timeout_min: 299,
        ..Default::default()
    };

    let config = Arc::new(config.validate().unwrap());
    let dir = Path::new("/tmp");
    let (log_store, state_machine_store) = new_storage(&dir).await;
    let kvs = state_machine_store.data.kvs.clone();

    let node_id = 1;
    let network = Network::new(client_poll);
    let raft = openraft::Raft::new(
        node_id,
        config.clone(),
        network,
        log_store,
        state_machine_store,
    )
    .await
    .unwrap();
    return raft;
}
