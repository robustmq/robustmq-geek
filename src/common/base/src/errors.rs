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

/*
 * Copyright (c) 2023 RobustMQ Team
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::io;
use tonic::Status;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RobustMQError {
    #[error("io error")]
    IOJsonError(#[from] io::Error),

    #[error("{0}")]
    CommmonError(String),

    #[error("{0}")]
    RocksdbError(#[from] rocksdb::Error),

    #[error("No available nodes in the cluster")]
    ClusterNoAvailableNode,

    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Description The interface {0} submitted logs to the commit log")]
    RaftLogCommitTimeout(String),

    #[error("{0} connection pool has no connection information available. {1}")]
    NoAvailableGrpcConnection(String, String),

    #[error("Grpc call of the node failed,Grpc status was {0}")]
    GrpcServerStatus(Status),
}
