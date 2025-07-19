use std::time::SystemTime;

use serde_json::json;
use socketioxide::extract::SocketRef;

use crate::{server::BCServer, utilities::millis_timestamps::SystemTimeMillisTimestamps};

impl BCServer {
    pub async fn emit_server_info(&self, socket: SocketRef) {
        // Sends the info to all players

        let server_info;
        {
            let accounts = self.accounts.lock().await;
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
}
