mod account_create;
mod account_login;
mod account_update;
mod send_server_info;
use axum::extract::ConnectInfo;
use socketioxide::{
    SocketIo,
    extract::{HttpExtension, SocketRef},
};
use std::{net::SocketAddr, sync::Arc};

use crate::{
    common::config::types::State,
    network::handlers::{
        account_create::on_account_create, account_login::on_account_login,
        account_update::account_update,
    },
};

pub fn register(io: &SocketIo, state: Arc<State>) {
    io.ns(
        "/",
        |socket: SocketRef, client_ip: HttpExtension<ConnectInfo<SocketAddr>>| {
            println!("Connected: {}", socket.id);
            on_connect(socket, client_ip, state);
        },
    )
    // socket.on("AccountLogin", function (data) { AccountLogin(data, socket); });
    // socket.on("PasswordReset", function(data) { PasswordReset(data, socket); });
    // socket.on("PasswordResetProcess", function(data) { PasswordResetProcess(data, socket); });
}

fn on_connect(
    socket: SocketRef,
    client_ip: HttpExtension<ConnectInfo<SocketAddr>>,
    state: Arc<State>,
) {
    let ip = client_ip.ip();
    let port = client_ip.port();
    let _socket_id = socket.id;

    println!("Connected: {ip}:{port}");
    socket.on_disconnect(move || {
        println!("Disconnected: {ip}:{port}");
    });
    let _ = socket.emit("message", "Welcum to Bondage Club!");

    // FIXME:
    let clone_state: Arc<State> = state.clone();
    socket.on("AccountCreate", async |data, socket, client_ip| {
        on_account_create(data, socket, clone_state, client_ip).await;
    });
    let clone_state: Arc<State> = state.clone();
    socket.on("AccountLogin", async |data, socket| {
        on_account_login(data, socket, clone_state).await;
    });

    socket.on("AccountUpdate", async |data, socket| {
        account_update(data, socket, state).await;
    });
}
