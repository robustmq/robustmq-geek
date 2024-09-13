use log::info;
use server::{
    grpc::server::start_grpc_server,
    http::server::{start_http_server, HttpServerState},
};
use tokio::{signal, sync::broadcast};

pub mod requests;
pub mod server;
pub mod storage;
pub mod raft;

pub async fn start_server(stop_sx: broadcast::Sender<bool>) {
    let raw_stop_sx = stop_sx.clone();
    tokio::spawn(async move {
        start_grpc_server(raw_stop_sx).await;
    });

    let raw_stop_sx = stop_sx.clone();
    tokio::spawn(async move {
        let state = HttpServerState::new();
        start_http_server(state, raw_stop_sx).await;
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
