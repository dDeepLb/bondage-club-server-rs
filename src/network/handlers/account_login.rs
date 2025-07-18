use std::{sync::Arc, time::SystemTime};

use crate::{
    common::{
        config::{
            self, SERVER_ACCOUNT_NAME_REGEX, SERVER_ACCOUNT_PASSWORD_REGEX,
            types::{Account, LoginQueueStruct, State},
        },
        utility::Utils,
    },
    network::handlers::send_server_info::account_send_server_info,
};
use async_recursion::async_recursion;
use mongodb::bson::{Bson, doc};
use serde::Deserialize;
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct AccountLoginRequest {
    account_name: String,
    password: String,
}

pub async fn on_account_login(Data(data): Data<Value>, socket: SocketRef, state: Arc<State>) {
    let data_clone = data.clone();
    let parsed = match serde_json::from_value::<AccountLoginRequest>(data) {
        Ok(p) => p,
        Err(err) => {
            println!("AccountLogin: Invalid payload: {err} | Raw: {data_clone}");
            let _ = socket.emit("LoginResponse", "Invalid request data");
            return;
        }
    };

    if !SERVER_ACCOUNT_NAME_REGEX.is_match(&parsed.account_name) {
        println!(
            "AccountCreate: Invalid AccountName: {}",
            parsed.account_name
        );
        let _ = socket.emit("CreationResponse", "Invalid account name");
        return;
    }

    if !SERVER_ACCOUNT_PASSWORD_REGEX.is_match(&parsed.password) {
        println!("AccountCreate: Invalid Password");
        let _ = socket.emit("CreationResponse", "Invalid password");
        return;
    }

    let uppercase_account_name = parsed.account_name.to_uppercase();
    let should_run;
    {
        let mut pending_logins = state.pending_logins.write().await;
        let mut login_queue = state.login_queue.write().await;
        // If connection already has login queued, ignore it
        if pending_logins.contains(&socket.id) {
            return;
        };
        should_run = login_queue.is_empty();
        login_queue.insert(
            socket.id,
            LoginQueueStruct {
                socket: socket.clone(),
                account_name: uppercase_account_name,
                password: parsed.clone().password,
            },
        );
        pending_logins.insert(socket.id);

        if login_queue.len() > 16 {
            let _ = socket.emit("LoginQueue", &login_queue.len());
        }
    }
    // If there are no logins being processed, start the processing of the queue
    if should_run {
        account_login_run(state).await;
    }
}

#[async_recursion]
async fn account_login_run(state: Arc<State>) {
    {
        let mut login_queue = state.login_queue.write().await;
        let mut pending_logins = state.pending_logins.write().await;
        // Get next waiting login
        if login_queue.is_empty() {
            return;
        };
        let mut next = login_queue.get(&pending_logins[0]).unwrap().clone();

        while !next.socket.connected() {
            // pop the first
            pending_logins.remove(&next.socket.id);
            login_queue.remove(&next.socket.id);
            if login_queue.is_empty() {
                return;
            }
            next = login_queue.get(&pending_logins[0]).unwrap().clone();
        }
        account_login_process(
            next.socket.clone(),
            next.account_name,
            next.password,
            state.clone(),
        )
        .await;

        pending_logins.remove(&next.socket.id);
        login_queue.remove(&next.socket.id);

        if !login_queue.is_empty() {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            account_login_run(state.clone()).await;
        }
    }
}

async fn account_login_process(
    socket: SocketRef,
    account_name: String,
    password: String,
    state: Arc<State>,
) {
    let config = config::load_config();
    let users: mongodb::Collection<Account> = state.db.collection(&config.db_collection);

    let account_result = users
        .find_one(doc! { "AccountName": account_name }, None)
        .await;
    if !socket.connected() {
        return;
    }
    if account_result.is_err() {
        let error = account_result.unwrap_err();
        println!("MongoDB error while checking existing account: {error}");
        let _ = socket.emit("LoginResponse", "ServerError");
        return;
    }

    if !socket.connected() {
        return;
    }
    let account_result = account_result.unwrap();
    if account_result.is_none() {
        let _ = socket.emit("LoginResponse", "InvalidNamePassword");
        return;
    }

    let mut account_result = account_result.unwrap();

    // Compare the password to its hashed version
    let password_result = match bcrypt::verify(
        password.to_uppercase(),
        &account_result.password.clone().unwrap(),
    ) {
        Ok(res) => res,
        Err(_) => {
            println!("Password hashing failed");
            let _ = socket.emit("LoginResponse", "ServerError");
            return;
        }
    };

    if !socket.connected() {
        return;
    }
    if !password_result {
        let _ = socket.emit("LoginResponse", "InvalidNamePassword");
        return;
    }

    // Disconnect duplicated logged accounts
    // FIXME: literally don't know, built on hopes
    {
        let mut accounts = state.accounts.write().await;
        for (index, account) in accounts.iter().enumerate() {
            if account.account_name == account_result.account_name {
                if account.socket.is_none() {
                    continue;
                }
                let socket = account.socket.as_ref().unwrap();
                let _ = socket.emit("ForceDisconnect", "ErrorDuplicatedLogin");
                let _ = <socketioxide::extract::SocketRef as Clone>::clone(socket).disconnect();
                accounts.remove(index);
                break;
            }
        }
    }

    //if (!Array.isArray(result.Lovership)) result.Lovership = (result.Lovership != undefined) ? [result.Lovership] : [];

    // Sets the last login date
    account_result.last_login = SystemTime::now().get_timestamp_in_milliseconds();
    let _ = users
        .update_one(
            doc! { "AccountName": &account_result.account_name },
            doc! { "$set": { "LastLogin": Bson::from(account_result.last_login) } },
            None,
        )
        .await;
    account_result.id = Some(socket.id.to_string());
    account_result.environment = "PROD".to_string(); //AccountGetEnvironment(socket);
    // AccountValidData(account_result)
    // AccountRemoveFromChatRoom(account_result.MemberNumber);
    {
        let mut accounts = state.accounts.write().await;
        accounts.push(account_result.clone());
    }
    //OnLogin(socket);
    let _ = socket.emit("LoginResponse", &account_result);

    /* 	/** @type {Account|null} */
       Account.push(result);
       OnLogin(socket);
       delete result.Password;
       delete result.Email;
       result.Socket = socket;
       AccountPurgeInfo(result);
    */
    account_send_server_info(socket, state).await;
}
