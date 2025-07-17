use std::sync::Arc;

use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};

use crate::common::{
    config::{self, types::State, SERVER_CHAT_ROOM_LIMIT_MAX, SERVER_CHAT_ROOM_LIMIT_MIN, SERVER_CHAT_ROOM_NAME_REGEX},
    types::{ServerChatRoomData, ServerChatRoomGame, ServerChatRoomSettings, ServerChatRoomSpace},
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

    let key = parsed.name.to_lowercase();
    let chat_rooms = state.chat_rooms.read().await;

    if chat_rooms.contains_key(&key) {
        socket
            .emit("ChatRoomCreateResponse", "RoomAlreadyExist")
            .unwrap();
    }

    let limit = parsed.limit.clamp(SERVER_CHAT_ROOM_LIMIT_MIN, SERVER_CHAT_ROOM_LIMIT_MAX);
    
    ServerChatRoomSettings {
        name: parsed.name.clone(),
        description: parsed.description.clone(),
        admin: parsed.admin.clone(),
        whitelist: parsed.whitelist.clone(),
        ban: parsed.ban.clone(),
        background: parsed.background.clone(),
        limit,
        game: parsed.game.clone(),
        visibility: parsed.visibility.clone(),
        access: parsed.access.clone(),
        block_category: parsed.block_category.clone(),
        language: parsed.language.clone(),
        space: parsed.space.clone(),
        map_data: None,
        custom: parsed.custom.clone(),
    };
}
