use bincode::{deserialize, serialize};
use clients::{placement::openraft::OpenRaftServiceManager, poll::ClientPool};
use common_base::errors::RobustMQError;
use mobc::Connection;
use openraft::{
    error::{InstallSnapshotError, RPCError, RaftError},
    network::RPCOption,
    raft::{
        AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest,
        InstallSnapshotResponse, VoteRequest, VoteResponse,
    },
    RaftNetwork,
};
use protocol::openraft::{AppendRequest, SnapshotRequest};
use std::sync::Arc;

use crate::openraft::{error::to_error, raft_node::NodeId, typeconfig::TypeConfig};

pub struct NetworkConnection {
    addr: String,
    client_poll: Arc<ClientPool>,
    target: NodeId,
}
impl NetworkConnection {
    pub fn new(addr: String, client_poll: Arc<ClientPool>, target: NodeId) -> Self {
        return NetworkConnection {
            addr,
            client_poll,
            target,
        };
    }

    async fn c(&mut self) -> Result<Connection<OpenRaftServiceManager>, RobustMQError> {
        return Ok(self
            .client_poll
            .placement_center_openraft_services_client(self.addr.clone())
            .await?);
    }
}

#[allow(clippy::blocks_in_conditions)]
impl RaftNetwork<TypeConfig> for NetworkConnection {
    
    async fn append_entries(
        &mut self,
        req: AppendEntriesRequest<TypeConfig>,
        _option: RPCOption,
    ) -> Result<AppendEntriesResponse<TypeConfig>, RPCError<TypeConfig, RaftError<TypeConfig>>>
    {

        let mut c = match self.c().await {
            Ok(conn) => conn,
            Err(e) => return Err(to_error(e)),
        };

        let value = match serialize(&req) {
            Ok(data) => data,
            Err(e) => return Err(to_error(RobustMQError::CommmonError(e.to_string()))),
        };

        let request = AppendRequest { value };

        let reply = match c.append(request).await {
            Ok(reply) => reply.into_inner(),
            Err(e) => return Err(to_error(RobustMQError::CommmonError(e.to_string()))),
        };

        let result = match deserialize(&reply.value) {
            Ok(data) => data,
            Err(e) => return Err(to_error(RobustMQError::CommmonError(e.to_string()))),
        };

        return Ok(result);
    }

    async fn install_snapshot(
        &mut self,
        req: InstallSnapshotRequest<TypeConfig>,
        _option: RPCOption,
    ) -> Result<
        InstallSnapshotResponse<TypeConfig>,
        RPCError<TypeConfig, RaftError<TypeConfig, InstallSnapshotError>>,
    > {
        tracing::debug!(req = debug(&req), "install_snapshot");

        let mut c = match self.c().await {
            Ok(conn) => conn,
            Err(e) => return Err(to_error(e)),
        };

        let value = match serialize(&req) {
            Ok(data) => data,
            Err(e) => return Err(to_error(RobustMQError::CommmonError(e.to_string()))),
        };

        let request = SnapshotRequest { value };

        let reply = match c.snapshot(request).await {
            Ok(reply) => reply.into_inner(),
            Err(e) => return Err(to_error(RobustMQError::CommmonError(e.to_string()))),
        };
        let result = match deserialize(&reply.value) {
            Ok(data) => data,
            Err(e) => return Err(to_error(RobustMQError::CommmonError(e.to_string()))),
        };

        return Ok(result);
    }

    async fn vote(
        &mut self,
        req: VoteRequest<TypeConfig>,
        _option: RPCOption,
    ) -> Result<VoteResponse<TypeConfig>, RPCError<TypeConfig, RaftError<TypeConfig>>> {
        tracing::debug!(req = debug(&req), "vote");
        let mut c = match self.c().await {
            Ok(conn) => conn,
            Err(e) => return Err(to_error(e)),
        };

        let value = match serialize(&req) {
            Ok(data) => data,
            Err(e) => return Err(to_error(RobustMQError::CommmonError(e.to_string()))),
        };

        let request = protocol::openraft::VoteRequest { value };

        let reply = match c.vote(request).await {
            Ok(reply) => reply.into_inner(),
            Err(e) => return Err(to_error(RobustMQError::CommmonError(e.to_string()))),
        };
        let result = match deserialize(&reply.value) {
            Ok(data) => data,
            Err(e) => return Err(to_error(RobustMQError::CommmonError(e.to_string()))),
        };

        return Ok(result);
    }
}
