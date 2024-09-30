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

use super::apply::{StorageData, StorageDataType};
use crate::storage::{kv::KvStorage, rocksdb::RocksDBEngine};
use bincode::deserialize;
use common_base::errors::RobustMQError;
use prost::Message as _;
use protocol::kv::{DeleteRequest, SetRequest};
use std::sync::Arc;

pub struct DataRoute {
    rocksdb_engine_handler: Arc<RocksDBEngine>,
}

impl DataRoute {
    pub fn new(rocksdb_engine_handler: Arc<RocksDBEngine>) -> DataRoute {
        return DataRoute {
            rocksdb_engine_handler,
        };
    }

    //Receive write operations performed by the Raft state machine and write subsequent service data after Raft state machine synchronization is complete.
    pub fn route(&self, data: Vec<u8>) -> Result<(), RobustMQError> {
        let storage_data: StorageData = deserialize(data.as_ref()).unwrap();
        match storage_data.data_type {
            StorageDataType::KvSet => {
                let kv_storage = KvStorage::new(self.rocksdb_engine_handler.clone());
                let req: SetRequest = SetRequest::decode(data.as_ref()).unwrap();
                return kv_storage.set(req.key, req.value);
            }
            StorageDataType::KvDelete => {
                let kv_storage = KvStorage::new(self.rocksdb_engine_handler.clone());
                let req: DeleteRequest = DeleteRequest::decode(data.as_ref()).unwrap();
                return kv_storage.delete(req.key);
            }
        }
    }
}
