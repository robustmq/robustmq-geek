// Copyright 2023 RobustMQ Team
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

use crate::server::grpc::services_kv::GrpcBrokerServices;
use common_base::config::placement_center::placement_center_conf;
use log::info;
use protocol::kv::kv_service_server::KvServiceServer;
use tokio::{select, sync::broadcast};
use tonic::transport::Server;

pub async fn start_grpc_server(stop_sx: broadcast::Sender<bool>) {
    let config = placement_center_conf();
    let server = GrpcServer::new(config.grpc_port);
    server.start(stop_sx).await;
}

pub struct GrpcServer {
    port: usize,
}

impl GrpcServer {
    pub fn new(port: usize) -> Self {
        return Self { port };
    }
    pub async fn start(&self, stop_sx: broadcast::Sender<bool>) {
        let addr = format!("0.0.0.0:{}", self.port).parse().unwrap();
        info!("Broker Grpc Server start. port:{}", self.port);
        let service_handler = GrpcBrokerServices::new();
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
            val =  Server::builder().add_service(KvServiceServer::new(service_handler)).serve(addr)=>{
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
