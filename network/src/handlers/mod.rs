use db::State;
use socketioxide::{
    SocketIo,
    extract::{SocketRef},
};
use std::sync::Arc;

pub fn register(io: &SocketIo, _state: Arc<State>) {
    io.ns(
        "/",
        |socket: SocketRef| {
            on_connect(socket);
        },
    )
    // socket.on("AccountCreate", function (data) { AccountCreate(data, socket); });
    // socket.on("AccountLogin", function (data) { AccountLogin(data, socket); });
    // socket.on("PasswordReset", function(data) { PasswordReset(data, socket); });
    // socket.on("PasswordResetProcess", function(data) { PasswordResetProcess(data, socket); });
}

fn on_connect(socket: SocketRef) {
    // println!("client_ip: {:?}", client_ip.get_ref());

    let ip = "/* addr.ip().to_string() */";
    let _socket_id = socket.id;

    println!("Connected: {ip}");
    socket.on_disconnect(move || {
        println!("Disconnected: {ip}");
    });
}
