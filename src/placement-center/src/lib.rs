use log::info;
use requests::network;
use server::server;
pub mod server;
pub mod requests;

pub fn start_server() {
    server();
    network();
    info!("start server log")
}
