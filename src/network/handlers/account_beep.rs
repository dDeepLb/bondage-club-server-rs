use std::sync::Arc;

use serde::Deserialize;
use serde_json::{Value, json};
use socketioxide::extract::{Data, SocketRef};

use crate::common::config::types::State;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AccountBeepRequest {
    member_number: u32,
    chat_room_space: Option<String>,
    chat_room_name: Option<String>,
    private: Option<bool>,
    beep_type: Option<String>,
    message: Option<Value>,
}

pub async fn account_beep(Data(data): Data<Value>, socket: SocketRef, state: Arc<State>) {
    let data_clone = data.clone();
    let parsed = match serde_json::from_value::<AccountBeepRequest>(data) {
        Ok(p) => p,
        Err(err) => {
            println!("AccountBeep: Invalid payload: {err} | Raw: {data_clone}");
            let _ = socket.emit("AccountBeep", "Invalid request data");
            return;
        }
    };

    {
        let accounts = state.accounts.read().await;
        let account = accounts
            .iter()
            .find(|a| a.id == Some(socket.id.to_string()));
        if account.is_none() {
            return;
        }
        let account = account.unwrap();

        let target = accounts
            .iter()
            .find(|a| a.member_number == parsed.member_number);
        if target.is_none() {
            return;
        }
        let target = target.unwrap();

        if
        /* target.environment != account.unwrap().environment  ||*/
        target.friend_list.is_empty()
            || !target.friend_list.contains(&account.member_number)
            || target.ownership.is_none()
            || (parsed.beep_type.is_some() && parsed.beep_type.clone().unwrap() != "Leash")
        {
            return;
        }

        if let Some(ownership) = target.ownership.as_ref() {
            if ownership.member_number != account.member_number {
                return;
            }
        }

        let _ = target.socket.as_ref().unwrap().emit(
            "AccountBeep",
            &json!({
                "MemberNumber": account.member_number,
                "MemberName": account.name,
                // todo: Chatroom
                // "ChatRoomSpace": account.chat_room.as_ref().unwrap().space,
                // "ChatRoomName": account.chat_room.as_ref().unwrap().name,
                // "Private": account.chat_room.as_ref().unwrap().private,
                "BeepType": parsed.beep_type,
                "Message": parsed.message,
            }),
        );
    }
}
