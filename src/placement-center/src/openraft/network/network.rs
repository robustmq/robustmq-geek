use clients::poll::ClientPool;
use log::info;
use openraft::RaftNetworkFactory;
use std::sync::Arc;

use super::connection::NetworkConnection;
use crate::openraft::{
    raft_node::{Node, NodeId},
    typeconfig::TypeConfig,
};

pub struct Network {
    client_poll: Arc<ClientPool>,
}

impl Network {
    pub fn new(client_poll: Arc<ClientPool>) -> Network {
        return Network { client_poll };
    }
}

// NOTE: This could be implemented also on `Arc<ExampleNetwork>`, but since it's empty, implemented
// directly.
impl RaftNetworkFactory<TypeConfig> for Network {
    type Network = NetworkConnection;

    #[tracing::instrument(level = "debug", skip_all)]
    async fn new_client(&mut self, target: NodeId, node: &Node) -> Self::Network {
        let addr = format!("{}", node.rpc_addr);
        return NetworkConnection::new(addr, self.client_poll.clone(), target);
    }
}
