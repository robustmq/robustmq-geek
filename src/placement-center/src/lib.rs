use server::http::server::{start_http_server, HttpServerState};
use tokio::sync::broadcast;
pub mod requests;
pub mod server;

pub async fn start_server(stop_sx: broadcast::Sender<bool>) {
    let state = HttpServerState::new();
    start_http_server(state, stop_sx).await;
}
