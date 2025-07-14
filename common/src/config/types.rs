use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server_addr: String,
    pub db_uri: String,
    pub db_name: String,
    pub db_collection: String
}
