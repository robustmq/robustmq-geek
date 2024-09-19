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

use std::sync::{Arc, RwLock};

use common_base::config::placement_center::placement_center_conf;
use log::info;
use raft::{
    apply::{RaftMachineApply, RaftMessage},
    machine::RaftMachine,
    metadata::RaftGroupMetadata,
    peer::PeerMessage,
    route::DataRoute,
};
use server::{
    grpc::server::start_grpc_server,
    http::server::{start_http_server, HttpServerState},
};
use storage::{raft::RaftMachineStorage, rocksdb::RocksDBEngine};
use tokio::{
    signal,
    sync::{broadcast, mpsc},
};

pub mod raft;
pub mod requests;
pub mod server;
pub mod storage;

pub async fn start_server(stop_sx: broadcast::Sender<bool>) {
    let config = placement_center_conf();
    let (raft_message_send, raft_message_recv) = mpsc::channel::<RaftMessage>(1000);
    let (peer_message_send, _) = mpsc::channel::<PeerMessage>(1000);

    let placement_cache = Arc::new(RwLock::new(RaftGroupMetadata::new()));

    let placement_center_storage = Arc::new(RaftMachineApply::new(raft_message_send));
    let rocksdb_engine_handler: Arc<RocksDBEngine> = Arc::new(RocksDBEngine::new(&config));

    let raft_machine_storage = Arc::new(RwLock::new(RaftMachineStorage::new(
        rocksdb_engine_handler.clone(),
    )));

    let raw_stop_sx = stop_sx.clone();
    tokio::spawn(async move {
        start_grpc_server(placement_center_storage, raw_stop_sx).await;
    });

    let raw_stop_sx = stop_sx.clone();
    tokio::spawn(async move {
        let state = HttpServerState::new();
        start_http_server(state, raw_stop_sx).await;
    });

    let data_route = Arc::new(RwLock::new(DataRoute::new(rocksdb_engine_handler.clone())));

    let mut raft: RaftMachine = RaftMachine::new(
        placement_cache.clone(),
        data_route,
        peer_message_send,
        raft_message_recv,
        stop_sx.subscribe(),
        raft_machine_storage.clone(),
    );

    tokio::spawn(async move {
        raft.run().await;
    });

    awaiting_stop(stop_sx.clone()).await;
}

pub async fn awaiting_stop(stop_send: broadcast::Sender<bool>) {
    signal::ctrl_c().await.expect("failed to listen for event");
    match stop_send.send(true) {
        Ok(_) => {
            info!(
                "{}",
                "When ctrl + c is received, the service starts to stop"
            );
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}
