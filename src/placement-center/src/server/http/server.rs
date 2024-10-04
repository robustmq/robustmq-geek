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

use crate::openraft::typeconfig::TypeConfig;

use super::openraft::{add_leadrner, change_membership, init, kv_get, metrics, set};
use super::path_list;
use super::{index::index, v1_path};
use axum::routing::{get, post};
use axum::Router;
use common_base::config::placement_center::placement_center_conf;
use log::info;
use openraft::Raft;
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::{select, sync::broadcast};

pub const ROUTE_ROOT: &str = "/index";
pub const ROUTE_ADD_LEARNER: &str = "/add-learner";
pub const ROUTE_CHANGE_MEMBERSHIP: &str = "/change-membership";
pub const ROUTE_INIT: &str = "/init";
pub const ROUTE_METRICS: &str = "/metrics";
pub const ROUTE_SET: &str = "/set";
pub const ROUTE_GET: &str = "/get";

#[derive(Clone)]
pub struct HttpServerState {
    pub raft_node: Raft<TypeConfig>,
    pub kvs: Arc<RwLock<BTreeMap<String, String>>>,
}

impl HttpServerState {
    pub fn new(raft_node: Raft<TypeConfig>, kvs: Arc<RwLock<BTreeMap<String, String>>>) -> Self {
        return Self { raft_node, kvs };
    }
}

pub async fn start_http_server(state: HttpServerState, stop_sx: broadcast::Sender<bool>) {
    let config = placement_center_conf();
    let ip: SocketAddr = match format!("0.0.0.0:{}", config.http_port).parse() {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e);
        }
    };

    info!("Broker HTTP Server start. port:{}", config.http_port);
    let app = routes(state);

    let mut stop_rx = stop_sx.subscribe();

    let listener = match tokio::net::TcpListener::bind(ip).await {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e);
        }
    };

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
        val = axum::serve(listener, app.clone())=>{
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

fn routes(state: HttpServerState) -> Router {
    let common = Router::new()
        .route(&v1_path(&path_list(ROUTE_ROOT)), get(index))
        .route(&v1_path(ROUTE_ADD_LEARNER), post(add_leadrner))
        .route(&v1_path(ROUTE_CHANGE_MEMBERSHIP), post(change_membership))
        .route(&v1_path(ROUTE_INIT), post(init))
        .route(&v1_path(ROUTE_METRICS), get(metrics))
        .route(&v1_path(ROUTE_SET), get(set))
        .route(&v1_path(ROUTE_GET), get(kv_get));

    let app = Router::new().merge(common);
    return app.with_state(state);
}
