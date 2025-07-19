use axum::{Router, extract::ConnectInfo, routing::get};
use axum_client_ip::ClientIpSource;
use socketioxide::SocketIo;
use std::{net::SocketAddr, time::Duration};
use tower_http::cors::{Any, CorsLayer};

pub mod handlers;

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
