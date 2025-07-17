use crate::common::{config::types::Account};
use serde::{Deserialize, Serialize};
use utility_types::Omit;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerChatRoomSpace {
    X,
    F,
    M,
    Asylum,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerChatRoomLanguage {
    EN,
    DE,
    FR,
    ES,
    CN,
    RU,
    UA,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerChatRoomRole {
    All,
    Admin,
    Whitelist,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerChatRoomGame {
    None,
    ClubCard,
    #[serde(rename = "LARP")]
    Larp,
    MagicBattle,
    #[serde(rename = "GGTS")]
    Ggts,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerChatRoomBlockCategory {
    // AssetCategory
    Medical,
    Extreme,
    Pony,
    SciFi,
    #[serde(rename = "ABDL")]
    Abdl,
    Fantasy,
    // Room features
    Leashing,
    Photos,
    Arousal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Omit)]
#[omit(arg(ident = ServerChatRoomSettings, fields(character), derive(Debug, PartialEq, Serialize, Deserialize, Clone)), forward_attrs(serde))]
#[serde(rename_all = "PascalCase")]
pub struct ServerChatRoomData {
    pub name: String,
    pub description: String,
    pub admin: Vec<u32>,
    pub whitelist: Vec<u32>,
    pub ban: Vec<u32>,
    pub background: String,
    pub limit: u8,
    pub game: ServerChatRoomGame,
    pub visibility: Vec<ServerChatRoomRole>,
    pub access: Vec<ServerChatRoomRole>,

    pub block_category: Vec<ServerChatRoomBlockCategory>,
    pub language: ServerChatRoomLanguage,
    pub space: ServerChatRoomSpace,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_data: Option<ServerChatRoomMapData>,

    pub custom: ServerChatRoomCustomData,
    pub character: Vec<Account>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerChatRoomMapData {
    #[serde(rename = "Type")]
    pub map_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fog: Option<bool>,

    pub tiles: String,
    pub objects: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerChatRoomCustomData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_filter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_mode: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChatRoom {
    #[serde(rename = "ID")]
    pub id: String,
    pub creator: String,
    pub creator_member_number: u32,
    pub creation: u32,
    pub account: Vec<Account>,

    pub name: String,
    pub description: String,
    pub admin: Vec<u32>,
    pub whitelist: Vec<u32>,
    pub ban: Vec<u32>,
    pub background: String,
    pub limit: u8,
    pub game: ServerChatRoomGame,
    pub visibility: Vec<ServerChatRoomRole>,
    pub access: Vec<ServerChatRoomRole>,

    pub block_category: Vec<ServerChatRoomBlockCategory>,
    pub language: ServerChatRoomLanguage,
    pub space: ServerChatRoomSpace,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_data: Option<ServerChatRoomMapData>,

    pub custom: ServerChatRoomCustomData,
}
