use serde_json::json;
use socketioxide::extract::SocketRef;

use crate::{
    common::protocol::AccountBeepRequest,
    server::BCServer,
    utilities::socket_account::{get_account_from_member_number, get_account_from_socket},
};

impl BCServer {
    pub async fn on_account_beep(&self, socket: SocketRef, request: AccountBeepRequest) {
        {
            let player = get_account_from_socket(&socket).unwrap();
            let player = player.lock().unwrap();
            let target = get_account_from_member_number(&self.io, request.member_number).unwrap();
            let target = target.lock().unwrap();

            if
            /* target.environment != account.unwrap().environment  ||*/
            target.friend_list.is_empty()
                || !target.friend_list.contains(&player.member_number)
                || target.ownership.is_none()
                || (request.beep_type.is_some() && request.beep_type.clone().unwrap() != "Leash")
            {
                return;
            }

            if let Some(ownership) = target.ownership.as_ref() {
                if ownership.member_number != player.member_number {
                    return;
                }
            }

            let _ = target.socket.as_ref().unwrap().emit(
                "AccountBeep",
                &json!({
                    "MemberNumber": player.member_number,
                    "MemberName": player.name,
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
