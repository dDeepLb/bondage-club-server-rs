use serde_json::json;
use socketioxide::extract::SocketRef;

use crate::{common::protocol::AccountBeepRequest, server::BCServer};

impl BCServer {
    pub async fn on_account_beep(&self, socket: SocketRef, request: AccountBeepRequest) {
        {
            let accounts = self.accounts.lock().await;
            let account = accounts
                .iter()
                .find(|a| a.id == Some(socket.id.to_string()));
            if account.is_none() {
                return;
            }
            let account = account.unwrap();

            let target = accounts
                .iter()
                .find(|a| a.member_number == request.member_number);
            if target.is_none() {
                return;
            }
            let target = target.unwrap();

            if
            /* target.environment != account.unwrap().environment  ||*/
            target.friend_list.is_empty()
                || !target.friend_list.contains(&account.member_number)
                || target.ownership.is_none()
                || (request.beep_type.is_some() && request.beep_type.clone().unwrap() != "Leash")
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
                    "BeepType": request.beep_type,
                    "Message": request.message,
                }),
            );
        }
    }
}
