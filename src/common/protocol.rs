use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use utility_types::Partial;

// #[derive(Debug, Clone, Deserialize)]
// #[serde(tag = "event", content = "data")]
// pub enum ClientToServerEvent {
//     AccountCreate(AccountCreateRequest),
//     AccountUpdate(AccountUpdateRequest),
//     // Add more events here...
// }

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AccountCreateRequest {
    pub account_name: String,
    pub password: String,
    pub name: String,
    pub email: Option<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AccountLoginRequest {
    pub account_name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Partial, Debug)]
#[partial(ident = Account, derive(Debug, PartialEq), forward_attrs())]
#[serde(rename_all = "PascalCase")]
pub struct AccountUpdateRequest {
    pub name: Option<String>,
    pub item_permission: Option<u8>,
    pub friend_list: Option<HashSet<u32>>,
    pub white_list: Option<HashSet<u32>>,
    pub black_list: Option<HashSet<u32>>,
    pub creation: Option<i64>,
    pub last_login: Option<i64>,
    pub chat_room: Option<Value>,
    pub ownership: Option<Value>,
    //pub delayed_appearance_update: Option<Value>,
    //pub delayed_skill_update: Option<Value>,
    //pub delayed_game_update: Option<Value>,
    pub inventory_data: Option<Value>,
    pub arousal_settings: Option<Value>,
    pub online_shared_settings: Option<Value>,
    pub game: Option<Value>,
    pub map_data: Option<Value>,
    pub label_color: Option<Value>,
    pub appearance: Option<Value>,
    pub reputation: Option<Vec<String>>,
    pub description: Option<String>,
    pub block_items: Option<Value>,
    pub limited_items: Option<Value>,
    pub favorite_items: Option<Value>,
    //pub lovership: Option<Vec<Value>>,
    //pub lover: Option<String>,
    pub skill: Option<Value>,
    pub title: Option<String>,
    pub nickname: Option<Value>,
    pub crafting: Option<Value>,
    pub log: Option<Vec<Value>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountQueryRequest {
    pub query: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountBeepRequest {
    pub member_number: u32,
    // pub chat_room_space: Option<String>,
    // pub chat_room_name: Option<String>,
    // pub private: Option<bool>,
    pub beep_type: Option<String>,
    pub message: Option<Value>,
}
