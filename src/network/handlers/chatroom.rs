use std::sync::Arc;

use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};

use crate::common::{
    config::{
        self, types::State, SERVER_CHAT_ROOM_LIMIT_MAX, SERVER_CHAT_ROOM_LIMIT_MIN, SERVER_CHAT_ROOM_NAME_REGEX
    },
    types::{ChatRoom, ServerChatRoomSettings}, utility::generate_base64_id,
};

pub async fn create_chat_room(Data(data): Data<Value>, socket: SocketRef, state: Arc<State>) {
    let config = config::load_config();

    let data_clone = data.clone();
    let parsed = match serde_json::from_value::<ServerChatRoomSettings>(data) {
        Ok(p) => p,
        Err(err) => {
            println!("ChatRoomCreate: Invalid payload: {err} | Raw: {data_clone}");
            let _ = socket.emit("ChatRoomCreateResponse", "InvalidRoomData");
            return;
        }
    };

    let room_name_matches_regex = SERVER_CHAT_ROOM_NAME_REGEX.is_match(&parsed.name);
    let desc_length = parsed.description.chars().count();
    let bg_length = parsed.background.chars().count();

    if !room_name_matches_regex || desc_length > 100 || bg_length > 100 {
        let _ = socket.emit("ChatRoomCreateResponse", "InvalidRoomData");
    }

    //TODO
    /*
    var Acc = AccountGet(socket.id);
        if (Acc == null) {
            socket.emit("ChatRoomCreateResponse", "AccountError");
            return;
        }
    */

    let key = parsed.name.to_uppercase();
    let chat_rooms = state.chat_rooms.read().await;

    if chat_rooms.contains_key(&key) {
        socket
            .emit("ChatRoomCreateResponse", "RoomAlreadyExist")
            .unwrap();
    }

    let limit = parsed
        .limit
        .clamp(SERVER_CHAT_ROOM_LIMIT_MIN, SERVER_CHAT_ROOM_LIMIT_MAX);

    let chatroom = ChatRoom {
        id: generate_base64_id(),
        creator: todo!(),
        creator_member_number: todo!(),
        creation: todo!(),
        account: todo!(),
        name: parsed.name,
        description: parsed.description,
        admin: parsed.admin,
        whitelist: parsed.whitelist,
        ban: parsed.ban,
        background: parsed.background,
        limit,
        game: parsed.game,
        visibility: parsed.visibility,
        access: parsed.access,
        block_category: parsed.block_category,
        language: parsed.language,
        space: parsed.space,
        map_data: parsed.map_data,
        custom: parsed.custom,
    };
    state.chat_rooms.write().await.insert(key, chatroom);
    chatroom.account.write().await.insert(socket.id(), todo!());
    let _ = socket.emit("ChatRoomCreateResponse", "ChatRoomCreated");
}
