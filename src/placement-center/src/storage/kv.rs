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

use common_base::errors::RobustMQError;

use crate::storage::{
    engine::{
        engine_delete_by_cluster, engine_exists_by_cluster, engine_get_by_cluster,
        engine_save_by_cluster,
    },
    rocksdb::RocksDBEngine,
};
use std::sync::Arc;

pub struct KvStorage {
    rocksdb_engine_handler: Arc<RocksDBEngine>,
}

impl KvStorage {
    pub fn new(rocksdb_engine_handler: Arc<RocksDBEngine>) -> Self {
        KvStorage {
            rocksdb_engine_handler,
        }
    }

    pub fn set(&self, key: String, value: String) -> Result<(), RobustMQError> {
        return engine_save_by_cluster(self.rocksdb_engine_handler.clone(), key, value);
    }

    pub fn delete(&self, key: String) -> Result<(), RobustMQError> {
        return engine_delete_by_cluster(self.rocksdb_engine_handler.clone(), key);
    }

    pub fn get(&self, key: String) -> Result<Option<String>, RobustMQError> {
        match engine_get_by_cluster(self.rocksdb_engine_handler.clone(), key) {
            Ok(Some(data)) => match serde_json::from_slice::<String>(&data.data) {
                Ok(data) => {
                    return Ok(Some(data));
                }
                Err(e) => {
                    return Err(e.into());
                }
            },
            Ok(None) => {
                return Ok(None);
            }
            Err(e) => Err(e),
        }
    }

    pub fn exists(&self, key: String) -> Result<bool, RobustMQError> {
        return engine_exists_by_cluster(self.rocksdb_engine_handler.clone(), key);
    }
}
