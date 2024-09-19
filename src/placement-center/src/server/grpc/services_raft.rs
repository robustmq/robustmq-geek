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

use std::sync::Arc;

use crate::raft::apply::RaftMachineApply;
use common_base::errors::RobustMQError;
use prost::Message;
use protocol::placement::{
    placement_center_service_server::PlacementCenterService, SendRaftConfChangeReply,
    SendRaftConfChangeRequest, SendRaftMessageReply, SendRaftMessageRequest,
};
use raft::eraftpb::{ConfChange, Message as raftPreludeMessage};
use tonic::{Request, Response, Status};

pub struct GrpcRaftServices {
    placement_center_storage: Arc<RaftMachineApply>,
}

impl GrpcRaftServices {
    pub fn new(placement_center_storage: Arc<RaftMachineApply>) -> Self {
        return GrpcRaftServices {
            placement_center_storage,
        };
    }
}

#[tonic::async_trait]
impl PlacementCenterService for GrpcRaftServices {
    async fn send_raft_message(
        &self,
        request: Request<SendRaftMessageRequest>,
    ) -> Result<Response<SendRaftMessageReply>, Status> {
        let message = raftPreludeMessage::decode(request.into_inner().message.as_ref())
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        match self
            .placement_center_storage
            .apply_raft_message(message, "send_raft_message".to_string())
            .await
        {
            Ok(_) => return Ok(Response::new(SendRaftMessageReply::default())),
            Err(e) => {
                return Err(Status::cancelled(
                    RobustMQError::RaftLogCommitTimeout(e.to_string()).to_string(),
                ));
            }
        }
    }
    async fn send_raft_conf_change(
        &self,
        request: Request<SendRaftConfChangeRequest>,
    ) -> Result<Response<SendRaftConfChangeReply>, Status> {
        let change = ConfChange::decode(request.into_inner().message.as_ref())
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        match self
            .placement_center_storage
            .apply_conf_raft_message(change, "send_conf_raft_message".to_string())
            .await
        {
            Ok(_) => return Ok(Response::new(SendRaftConfChangeReply::default())),
            Err(e) => {
                return Err(Status::cancelled(
                    RobustMQError::RaftLogCommitTimeout(e.to_string()).to_string(),
                ));
            }
        }
    }
}
