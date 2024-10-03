// Copyright 2023 RobustMQ Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use bincode::{deserialize, serialize};
use openraft::Raft;
use protocol::openraft::{
    open_raft_service_server::OpenRaftService, AppendReply, AppendRequest, SnapshotReply,
    SnapshotRequest, VoteReply, VoteRequest,
};
use tonic::{Request, Response, Status};

use crate::openraft::typeconfig::TypeConfig;

pub struct GrpcOpenRaftServices {
    raft_node: Raft<TypeConfig>,
}

impl GrpcOpenRaftServices {
    pub fn new(raft_node: Raft<TypeConfig>) -> Self {
        return GrpcOpenRaftServices { raft_node };
    }
}

#[tonic::async_trait]
impl OpenRaftService for GrpcOpenRaftServices {
    async fn vote(&self, request: Request<VoteRequest>) -> Result<Response<VoteReply>, Status> {
        let req = request.into_inner();
        let vote_data = deserialize(&req.value).unwrap();
        let res = match self.raft_node.vote(vote_data).await {
            Ok(data) => data,
            Err(e) => {
                return Err(Status::cancelled(e.to_string()));
            }
        };

        let mut reply = VoteReply::default();
        reply.value = match serialize(&res) {
            Ok(data) => data,
            Err(e) => {
                return Err(Status::cancelled(e.to_string()));
            }
        };
        return Ok(Response::new(reply));
    }

    async fn append(
        &self,
        request: Request<AppendRequest>,
    ) -> Result<Response<AppendReply>, Status> {
        let req = request.into_inner();
        let vote_data = deserialize(&req.value).unwrap();
        let res = match self.raft_node.append_entries(vote_data).await {
            Ok(data) => data,
            Err(e) => {
                return Err(Status::cancelled(e.to_string()));
            }
        };

        let mut reply = AppendReply::default();
        reply.value = match serialize(&res) {
            Ok(data) => data,
            Err(e) => {
                return Err(Status::cancelled(e.to_string()));
            }
        };
        return Ok(Response::new(reply));
    }

    async fn snapshot(
        &self,
        request: Request<SnapshotRequest>,
    ) -> Result<Response<SnapshotReply>, Status> {
        let req = request.into_inner();
        let vote_data = deserialize(&req.value).unwrap();
        let res = match self.raft_node.install_snapshot(vote_data).await {
            Ok(data) => data,
            Err(e) => {
                return Err(Status::cancelled(e.to_string()));
            }
        };

        let mut reply = SnapshotReply::default();
        reply.value = match serialize(&res) {
            Ok(data) => data,
            Err(e) => {
                return Err(Status::cancelled(e.to_string()));
            }
        };
        return Ok(Response::new(reply));
    }
}
