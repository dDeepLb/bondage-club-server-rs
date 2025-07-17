use serde::{Deserialize, Serialize};
use utility_types::{Omit, Partial, Pick, Required};
use crate::common::macros::trim_string;

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
    LARP,
    MagicBattle,
    GGTS,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerChatRoomBlockCategory {
    // AssetCategory
    Medical,
    Extreme,
    Pony,
    SciFi,
    ABDL,
    Fantasy,
    // Room features
    Leashing,
    Photos,
    Arousal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Omit)]
#[omit(arg(ident = ServerChatRoomSettings, fields(character), derive(Debug, PartialEq, Serialize, Deserialize)), forward_attrs(serde))]
#[serde(rename_all = "PascalCase")]
pub struct ServerChatRoomData {

    #[serde(deserialize_with = "trim_string")]
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
    pub character: Vec<String>,
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

