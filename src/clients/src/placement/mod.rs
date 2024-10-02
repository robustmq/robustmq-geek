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

use crate::{poll::ClientPool, retry_sleep_time, retry_times};
use common_base::errors::RobustMQError;
use kv::kv_interface_call;
use log::error;
use openraft::openraft_interface_call;
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

#[derive(Clone, Debug)]
pub enum PlacementCenterService {
    Kv,
    OpenRaft,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PlacementCenterInterface {
    // kv interface
    Set,
    Get,
    Delete,
    Exists,

    // Open Raft
    Vote,
    Append,
    Snapshot,
}

pub mod kv;
pub mod openraft;

async fn retry_call(
    service: PlacementCenterService,
    interface: PlacementCenterInterface,
    client_poll: Arc<ClientPool>,
    addrs: Vec<String>,
    request: Vec<u8>,
) -> Result<Vec<u8>, RobustMQError> {
    let mut times = 1;
    loop {
        let index = times % addrs.len();
        let addr = addrs.get(index).unwrap().clone();
        let result = match service {
            PlacementCenterService::Kv => {
                kv_interface_call(
                    interface.clone(),
                    client_poll.clone(),
                    addr.clone(),
                    request.clone(),
                )
                .await
            }

            PlacementCenterService::OpenRaft => {
                openraft_interface_call(
                    interface.clone(),
                    client_poll.clone(),
                    addr.clone(),
                    request.clone(),
                )
                .await
            }
        };

        match result {
            Ok(data) => {
                return Ok(data);
            }
            Err(e) => {
                error!(
                    "{:?}@{:?}@{},{},",
                    service.clone(),
                    interface.clone(),
                    addr.clone(),
                    e
                );
                if times > retry_times() {
                    return Err(e);
                }
                times = times + 1;
            }
        }
        sleep(Duration::from_secs(retry_sleep_time(times) as u64)).await;
    }
}
