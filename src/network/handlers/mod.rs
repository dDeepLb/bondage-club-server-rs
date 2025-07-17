mod account;
use axum::extract::ConnectInfo;
use socketioxide::{
    extract::{Data, HttpExtension, SocketRef}, SocketIo
};
use std::{net::SocketAddr, sync::Arc};

use crate::{common::config::types::State, network::handlers::account::{on_account_create, AccountCreateRequest}};

use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AccountUpdateRequest {

}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum ClientToServerEvent {
    AccountCreate(AccountCreateRequest),
    AccountUpdate(AccountUpdateRequest),
    // Add more events here...
}

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
    socket.on("AccountCreate", async |Data(data): Data<AccountCreateRequest>, socket, client_ip| {
        on_account_create(data, socket, state, client_ip).await;
    });
}
