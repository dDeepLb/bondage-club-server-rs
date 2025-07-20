use std::sync::{Arc, Mutex};

use socketioxide::{SocketIo, extract::SocketRef};

use crate::models::account::Account;

pub async fn attach_account_to_socket(socket: &SocketRef, account: Account) {
    let mut parts = socket.req_parts().clone();
    let mut_account = &mut Arc::new(account);
    parts
        .extensions
        .get_mut::<Arc<Account>>()
        .insert(mut_account);
}

pub fn get_account_from_socket(socket: &SocketRef) -> Option<Arc<Mutex<Account>>> {
    socket
        .req_parts()
        .extensions
        .get::<Arc<Mutex<Account>>>()
        .cloned()
}

pub fn get_sockets(io: &SocketIo) -> Vec<SocketRef> {
    io.sockets()
}

pub fn get_accounts(io: &SocketIo) -> Vec<Arc<Mutex<Account>>> {
    get_sockets(io)
        .iter()
        .filter_map(|socket| get_account_from_socket(socket))
        .collect()
}

pub fn get_account_from_member_number(
    io: &SocketIo,
    member_number: u32,
) -> Option<Arc<Mutex<Account>>> {
    get_accounts(io)
        .iter()
        .find(|a| a.lock().unwrap().member_number == member_number)
        .cloned()
}
