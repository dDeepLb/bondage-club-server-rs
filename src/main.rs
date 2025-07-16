use std::net::SocketAddr;

use axum::{Router, serve};
use tracing::info;

mod common;
mod db;
mod network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load .env, config, logging, etc.
    let config = common::config::load_config();
    tracing_subscriber::fmt().init();

    let db_uri: &str = &config.db_uri;
    let db_name: &str = &config.db_name;
    let db_collection: &str = &config.db_collection;

    let state = db::setup_mongodb(db_uri, db_name, db_collection).await;
    // initialize Socket.IO
    let (socket_router, io) = network::init_socket_io();

    network::handlers::register(&io, state.clone());

    let app = Router::new()
        .merge(socket_router)
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = config.server_addr;
    let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
    info!("listening on {addr}");
    serve(listener, app).await?;

    Ok(())
}
