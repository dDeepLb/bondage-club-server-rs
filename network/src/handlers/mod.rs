use axum::extract::ConnectInfo;
use db::State;
use socketioxide::{
    extract::{HttpExtension, SocketRef}, SocketIo
};
use std::{net::SocketAddr, sync::Arc};

pub fn register(io: &SocketIo, _state: Arc<State>) {
    io.ns(
        "/",
        |socket: SocketRef, client_ip: HttpExtension<ConnectInfo<SocketAddr>>| {
            on_connect(socket, client_ip);
        },
    )
    // socket.on("AccountCreate", function (data) { AccountCreate(data, socket); });
    // socket.on("AccountLogin", function (data) { AccountLogin(data, socket); });
    // socket.on("PasswordReset", function(data) { PasswordReset(data, socket); });
    // socket.on("PasswordResetProcess", function(data) { PasswordResetProcess(data, socket); });
}

fn on_connect(socket: SocketRef, client_ip: HttpExtension<ConnectInfo<SocketAddr>>) {
    let ip = client_ip.ip();
    let port = client_ip.port();
    let _socket_id = socket.id;

    println!("Connected: {ip}:{port}");
    socket.on_disconnect(move || {
        println!("Disconnected: {ip}:{port}");
    });
}
