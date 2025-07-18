use std::{net::IpAddr, time::SystemTime};

use mongodb::Database;
use ordermap::{OrderMap, OrderSet};
use serde::{self, Deserialize, Serialize};
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
    pub friend_list: Vec<String>,
    pub white_list: Vec<String>,
    pub black_list: Vec<String>,
    pub money: u32,
    pub creation: i64,
    pub last_login: i64,
    pub environment: String, // "PROD" | "DEV" | string;
    #[serde(skip)]
    pub socket: Option<SocketRef>,
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
