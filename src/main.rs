use crate::server::{BCServer, load_config};
use axum::{Router, extract::ConnectInfo, routing::get, serve};
use axum_client_ip::ClientIpSource;
use mongodb::{Client, options::ClientOptions};
use socketioxide::SocketIo;
use std::{net::SocketAddr, time::Duration};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

mod common;
mod handlers;
mod models;
mod server;
mod utilities;

async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    addr.ip().to_string()
}

pub fn init_socket_io() -> (Router, SocketIo) {
    let (layer, io) = SocketIo::builder()
        .max_buffer_size(180_000)
        .ping_timeout(Duration::new(30, 0))
        .ping_interval(Duration::new(50, 0))
        .connect_timeout(Duration::new(5, 0))
        .build_layer();

    let router = Router::new()
        .route("/", get(handler))
        .layer(layer)
        .layer(ClientIpSource::ConnectInfo.into_extension())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    (router, io)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load .env, config, logging, etc.
    let config = load_config();
    tracing_subscriber::fmt().init();

    let db_uri: &str = &config.db_uri;
    let db_name: &str = &config.db_name;

    let options = ClientOptions::parse(db_uri).await.unwrap();
    let client = Client::with_options(options).unwrap();
    let db = client.database(db_name);

    let (socket_router, io) = init_socket_io();

    let _server = BCServer::new(db, io).await;

    let app = Router::new()
        .merge(socket_router)
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = config.server_addr;
    let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
    info!("listening on {addr}");
    serve(listener, app).await?;

    Ok(())
}
