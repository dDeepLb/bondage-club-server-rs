use std::collections::HashSet;

// use mongodb::Database;
// use ordermap::{OrderMap, OrderSet};
// use serde::{self, Deserialize, Serialize};
use serde_json::Value;
// use socketioxide::{extract::SocketRef, socket::Sid};
// use tokio::sync::RwLock;

use crate::common::constants::SERVER_ACCOUNT_EMAIL_REGEX;
use serde::{Deserialize, Serialize};
use socketioxide::extract::SocketRef;

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

impl Account {
    pub fn is_valid_mail(mail: &str) -> bool {
        if !SERVER_ACCOUNT_EMAIL_REGEX.is_match(mail) {
            return false;
        }

        let parts: Vec<&str> = mail.split("@").collect();

        if parts.len() != 2 {
            return false;
        };
        if !parts[1].contains(".") {
            return false;
        };
        true
    }
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
