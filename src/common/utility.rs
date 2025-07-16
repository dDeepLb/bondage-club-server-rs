use std::time::{SystemTime, UNIX_EPOCH};

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
    fn get_timestamp_in_milliseconds(&self) -> u128;
}

impl Utils for SystemTime {
    fn get_timestamp_in_milliseconds(&self) -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0)
    }
}
