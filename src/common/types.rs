use socketioxide::extract::SocketRef;
use std::{net::IpAddr, time::SystemTime};

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
