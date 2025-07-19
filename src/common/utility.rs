#![allow(clippy::let_underscore_drop)] 
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};

use socketioxide::extract::SocketRef;

use crate::common::config::{types::Account, SERVER_ACCOUNT_EMAIL_REGEX};

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

pub async fn attach_account_to_socket(socket: &SocketRef, account: Account) {
    let mut parts = socket.req_parts().clone();
    let mut_account = &mut Arc::new(account);
    parts.extensions.get_mut::<Arc<Account>>().insert(mut_account);
}

pub fn get_account_from_socket(socket: &SocketRef) -> Option<Arc<Mutex<Account>>> {
    socket.req_parts().extensions.get::<Arc<Mutex<Account>>>().cloned()
}
