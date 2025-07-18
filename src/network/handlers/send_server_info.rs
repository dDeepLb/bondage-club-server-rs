use std::{sync::Arc, time::SystemTime};

use crate::common::{
    config::types::{Account, State},
    utility::Utils,
};

use serde_json::json;
use socketioxide::extract::SocketRef;

pub async fn account_send_server_info(socket: SocketRef, state: Arc<State>) {
    // Sends the info to all players

    let server_info;
    let read_state = state.clone();
    {
        let accounts: tokio::sync::RwLockReadGuard<'_, Vec<Account>> =
            read_state.accounts.read().await;
        server_info = json!( {
            "Time": SystemTime::now().get_timestamp_in_milliseconds(),
            "OnlinePlayers": accounts.len(),
        })
    }
    //if socket != None {
    let _ = socket.emit("ServerInfo", &server_info);
    // } else {
    //IO.sockets.volatile.emit("ServerInfo", server_info);
    //}
}
