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

use crate::{
    openraft::typeconfig::TypeConfig,
    raft::{apply::RaftMachineApply, metadata::RaftGroupMetadata},
    server::grpc::{
        services_kv::GrpcKvServices, services_openraft::GrpcOpenRaftServices,
        services_raft::GrpcRaftServices,
    },
    storage::rocksdb::RocksDBEngine,
};
use clients::poll::ClientPool;
use common_base::config::placement_center::placement_center_conf;
use log::info;
use openraft::Raft;
use protocol::{
    kv::kv_service_server::KvServiceServer,
    openraft::open_raft_service_server::OpenRaftServiceServer,
    placement::placement_center_service_server::PlacementCenterServiceServer,
};
use std::sync::{Arc, RwLock};
use tokio::{select, sync::broadcast};
use tonic::transport::Server;

pub async fn start_grpc_server(
    client_poll: Arc<ClientPool>,
    raft_node: Raft<TypeConfig>,
    placement_center_storage: Arc<RaftMachineApply>,
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    placement_cluster: Arc<RwLock<RaftGroupMetadata>>,
    stop_sx: broadcast::Sender<bool>,
) {
    let config = placement_center_conf();
    let server = GrpcServer::new(config.grpc_port);
    server
        .start(
            client_poll,
            placement_center_storage,
            rocksdb_engine_handler,
            placement_cluster,
            stop_sx,
            raft_node,
        )
        .await;
}

pub struct GrpcServer {
    port: usize,
}

impl GrpcServer {
    pub fn new(port: usize) -> Self {
        return Self { port };
    }
    pub async fn start(
        &self,
        client_poll: Arc<ClientPool>,
        placement_center_storage: Arc<RaftMachineApply>,
        rocksdb_engine_handler: Arc<RocksDBEngine>,
        placement_cluster: Arc<RwLock<RaftGroupMetadata>>,
        stop_sx: broadcast::Sender<bool>,
        raft_node: Raft<TypeConfig>,
    ) {
        let addr = format!("0.0.0.0:{}", self.port).parse().unwrap();
        info!("Broker Grpc Server start. port:{}", self.port);

        let kv_service_handler = GrpcKvServices::new(
            client_poll.clone(),
            placement_center_storage.clone(),
            rocksdb_engine_handler,
            placement_cluster,
        );
        let raft_service_handler = GrpcRaftServices::new(placement_center_storage);

        let openraft_service_handler = GrpcOpenRaftServices::new(raft_node);

        let mut stop_rx = stop_sx.subscribe();
        select! {

            val = stop_rx.recv() =>{
                match val{
                    Ok(flag) => {
                        if flag {
                            info!("HTTP Server stopped successfully");

                        }
                    }
                    Err(_) => {}
                }
            },

            val =  Server::builder().add_service(KvServiceServer::new(kv_service_handler))
                                    .add_service(PlacementCenterServiceServer::new(raft_service_handler))
                                    .add_service(OpenRaftServiceServer::new(openraft_service_handler))
                                    .serve(addr)=>{
                match val{
                    Ok(()) => {
                    },
                    Err(e) => {
                        panic!("{}",e);
                    }
                }
            }
        }
    }
}
