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

use crate::placement::{kv::KvServiceManager, openraft::OpenRaftServiceManager};
use common_base::errors::RobustMQError;
use dashmap::DashMap;
use mobc::{Connection, Pool};

#[derive(Clone)]
pub struct ClientPool {
    max_open_connection: u64,
    // placement center
    placement_center_kv_service_pools: DashMap<String, Pool<KvServiceManager>>,
    placement_center_openraft_service_pools: DashMap<String, Pool<OpenRaftServiceManager>>,
}

impl ClientPool {
    pub fn new(max_open_connection: u64) -> Self {
        Self {
            max_open_connection,
            placement_center_kv_service_pools: DashMap::with_capacity(2),
            placement_center_openraft_service_pools: DashMap::with_capacity(2),
        }
    }

    pub async fn placement_center_kv_services_client(
        &self,
        addr: String,
    ) -> Result<Connection<KvServiceManager>, RobustMQError> {
        let module = "KvServices".to_string();
        let key = format!("{}_{}_{}", "PlacementCenter", module, addr);

        if !self.placement_center_kv_service_pools.contains_key(&key) {
            let manager = KvServiceManager::new(addr.clone());
            let pool = Pool::builder()
                .max_open(self.max_open_connection)
                .build(manager);
            self.placement_center_kv_service_pools
                .insert(key.clone(), pool);
        }

        if let Some(poll) = self.placement_center_kv_service_pools.get(&key) {
            match poll.get().await {
                Ok(conn) => {
                    return Ok(conn);
                }
                Err(e) => {
                    return Err(RobustMQError::NoAvailableGrpcConnection(
                        module,
                        e.to_string(),
                    ));
                }
            };
        }

        return Err(RobustMQError::NoAvailableGrpcConnection(
            module,
            "connection pool is not initialized".to_string(),
        ));
    }

    pub async fn placement_center_openraft_services_client(
        &self,
        addr: String,
    ) -> Result<Connection<OpenRaftServiceManager>, RobustMQError> {
        let module = "OpenRaftServices".to_string();
        let key = format!("{}_{}_{}", "PlacementCenter", module, addr);

        if !self
            .placement_center_openraft_service_pools
            .contains_key(&key)
        {
            let manager = OpenRaftServiceManager::new(addr.clone());
            let pool = Pool::builder()
                .max_open(self.max_open_connection)
                .build(manager);
            self.placement_center_openraft_service_pools
                .insert(key.clone(), pool);
        }

        if let Some(poll) = self.placement_center_openraft_service_pools.get(&key) {
            match poll.get().await {
                Ok(conn) => {
                    return Ok(conn);
                }
                Err(e) => {
                    return Err(RobustMQError::NoAvailableGrpcConnection(
                        module,
                        e.to_string(),
                    ));
                }
            };
        }

        return Err(RobustMQError::NoAvailableGrpcConnection(
            module,
            "connection pool is not initialized".to_string(),
        ));
    }
}
