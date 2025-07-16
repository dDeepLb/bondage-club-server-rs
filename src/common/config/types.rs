use std::{net::IpAddr, time::SystemTime};

use mongodb::Database;
use serde::{self, Deserialize, Serialize};
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
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Account {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub account_name: String,
    pub name: String,
    pub password: Option<String>,
    pub email: Option<String>,
    pub member_number: u32,
    //Lovership: Vec<Lovership>,
    pub item_permission: u8,
    pub friend_list: Vec<String>,
    pub white_list: Vec<String>,
    pub black_list: Vec<String>,
    pub money: u32,
    pub creation: u128,
    pub last_login: u128,
    pub environment: String, // "PROD" | "DEV" | string;
                             //Socket ServerSocket??
                             //ChatRoom: Option<Chatroom>,
                             //Ownership: Option<ServerOwnership>,
                             //DelayedAppearanceUpdate: Option<ServerAccountData["Appearance"]>,
                             // DelayedSkillUpdate: Option<ServerAccountData["Skill"]>,
                             // DelayedGameUpdate: Option<ServerChatRoomGame>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AccountCreationIP {
    pub address: IpAddr,
    pub time: SystemTime,
}

#[derive(Debug)]
pub struct State {
    pub db: Database,
    pub next_member_number: RwLock<u32>,
    pub account_creation_ip: RwLock<Vec<AccountCreationIP>>,
    pub accounts: RwLock<Vec<Account>>,
}
