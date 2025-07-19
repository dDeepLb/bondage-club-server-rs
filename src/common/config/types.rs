use std::{collections::HashSet, net::IpAddr, time::SystemTime};

use mongodb::Database;
use ordermap::{OrderMap, OrderSet};
use serde::{self, Deserialize, Serialize};
use serde_json::Value;
use socketioxide::{extract::SocketRef, socket::Sid};
use tokio::sync::RwLock;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server_addr: String,
    pub db_uri: String,
    pub db_name: String,
    pub db_collection: String,
    pub max_ip_account_per_day: u32,
    pub max_ip_account_per_hour: u32,
}

/*
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct Lovership {
    pub member_number: <u32,
    Name?: Option<String>,
    Stage?: number;
    Start?: number;
    BeginDatingOfferedByMemberNumber?: number;
    BeginEngagementOfferedByMemberNumber?: number;
    BeginWeddingOfferedByMemberNumber?: number;
}
*/
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Ownership {
    pub name: String,
    pub member_number: u32,
    pub stage: u8,
    pub start: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Account {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub account_name: String,
    pub name: String,
    pub password: Option<String>,
    pub email: Option<String>,
    pub member_number: u32,
    //pub lovership: Vec<Lovership>,
    pub item_permission: u8,
    pub friend_list: HashSet<u32>,
    pub white_list: HashSet<u32>,
    pub black_list: HashSet<u32>,
    pub money: u32,
    pub creation: i64,
    pub last_login: i64,
    pub environment: String, // "PROD" | "DEV" | string;
    #[serde(skip)]
    pub socket: Option<SocketRef>,
    pub chat_room: Option<Value>,
    pub ownership: Option<Ownership>,
    pub delayed_appearance_update: Option<Value>,
    pub delayed_skill_update: Option<Value>,
    pub delayed_game_update: Option<Value>,
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
    pub lovership: Option<Vec<Value>>,
    pub lover: Option<String>,
    pub skill: Option<Value>,
    pub title: Option<String>,
    pub nickname: Option<Value>,
    pub crafting: Option<Value>,
    pub log: Option<Vec<Value>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AccountCreationIP {
    pub address: IpAddr,
    pub time: SystemTime,
}
#[derive(Debug, Clone)]
pub struct LoginQueueStruct {
    pub socket: SocketRef,
    pub account_name: String,
    pub password: String,
}
#[derive(Debug)]
pub struct State {
    pub db: Database,
    pub next_member_number: RwLock<u32>,
    pub account_creation_ip: RwLock<Vec<AccountCreationIP>>,
    pub accounts: RwLock<Vec<Account>>,
    pub login_queue: RwLock<OrderMap<Sid, LoginQueueStruct>>,
    pub pending_logins: RwLock<OrderSet<Sid>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerFriendInfo {
    pub r#type: &'static str, // "type" is a reserved keyword
    pub member_number: u32,
    pub member_name: String,
    // todo: Chatroom
    /*  chat_room_space: Option<String>,
    chat_room_name: Option<String>,
    private: Option<bool>, */
}
