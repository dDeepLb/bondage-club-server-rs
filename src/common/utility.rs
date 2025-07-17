use std::time::{SystemTime, UNIX_EPOCH};
use rand::RngCore;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

use crate::common::config::SERVER_ACCOUNT_EMAIL_REGEX;

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

pub trait Utils {
    fn get_timestamp_in_milliseconds(&self) -> i64;
}

impl Utils for SystemTime {
    fn get_timestamp_in_milliseconds(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0) as i64
    }
}

pub fn generate_base64_id() -> String {
    let mut bytes = [0u8; 12]; // 12 random bytes
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(&bytes)
}