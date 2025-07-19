use regex::Regex;
use std::sync::LazyLock;

pub static SERVER_ACCOUNT_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9]{1,20}$").unwrap());
pub static SERVER_CHARACTER_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_]{1,32}$").unwrap());
pub static SERVER_ACCOUNT_PASSWORD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9]{1,20}$").unwrap());
pub static SERVER_ACCOUNT_EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9@.!#$%&'*+/=?^_`{|}~-]{5,100}$").unwrap());
