use figment::{
    providers::{Env},
    Figment,
};
use dotenvy::dotenv;
use types::AppConfig;

mod types;

pub fn load_config() -> AppConfig {
    dotenv().ok();

    Figment::from(Env::prefixed("APP_"))
        .extract()
        .expect("Failed to load configuration")
}
