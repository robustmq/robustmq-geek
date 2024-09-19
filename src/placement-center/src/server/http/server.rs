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

use super::{index::index, v1_path};
use super::{path_create, path_delete, path_list, path_update};
use axum::routing::{delete, get, post, put};
use axum::Router;
use common_base::config::placement_center::placement_center_conf;
use log::info;
use std::net::SocketAddr;
use tokio::{select, sync::broadcast};

pub const ROUTE_ROOT: &str = "/index";

#[derive(Clone)]
pub struct HttpServerState {}

impl HttpServerState {
    pub fn new() -> Self {
        return Self {};
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
        .route(&v1_path(&path_create(ROUTE_ROOT)), post(index))
        .route(&v1_path(&path_update(ROUTE_ROOT)), put(index))
        .route(&v1_path(&path_delete(ROUTE_ROOT)), delete(index));

    let app = Router::new().merge(common);
    return app.with_state(state);
}
