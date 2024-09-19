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

use bincode::deserialize;
use common_base::errors::RobustMQError;
use std::sync::Arc;
use crate::storage::rocksdb::RocksDBEngine;
use super::{apply::{StorageData, StorageDataType}, kv::DataRouteKv};

pub struct DataRoute {
    route_kv: DataRouteKv,
}

impl DataRoute {
    pub fn new(
        rocksdb_engine_handler: Arc<RocksDBEngine>,
    ) -> DataRoute {
        let route_kv = DataRouteKv::new(rocksdb_engine_handler.clone());
        return DataRoute { route_kv };
    }

    //Receive write operations performed by the Raft state machine and write subsequent service data after Raft state machine synchronization is complete.
    pub fn route(&self, data: Vec<u8>) -> Result<(), RobustMQError> {
        let storage_data: StorageData = deserialize(data.as_ref()).unwrap();
        match storage_data.data_type {
            StorageDataType::KvSet => {
                return self.route_kv.set(storage_data.value);
            }
            StorageDataType::KvDelete => {
                return self.route_kv.delete(storage_data.value);
            }
        }
    }
}
