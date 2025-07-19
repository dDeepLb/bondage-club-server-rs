use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, SystemTime},
};

use crate::{
    common::{
        config::{
            self, SERVER_ACCOUNT_NAME_REGEX, SERVER_ACCOUNT_PASSWORD_REGEX,
            SERVER_CHARACTER_NAME_REGEX,
            types::{Account, AccountCreationIP, AppConfig, State},
        },
        utility::{Utils, is_valid_mail},
    },
    network::handlers::send_server_info::account_send_server_info,
};
use axum::extract::ConnectInfo;
use mongodb::bson::doc;
use serde::Deserialize;
use serde_json::{Value, json};
use socketioxide::extract::{Data, HttpExtension, SocketRef};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AccountCreateRequest {
    account_name: String,
    password: String,
    name: String,
    email: Option<String>,
}

pub async fn on_account_create(
    Data(data): Data<Value>,
    socket: SocketRef,
    state: Arc<State>,
    client_ip: HttpExtension<ConnectInfo<SocketAddr>>,
) {
    let config = config::load_config();

    let data_clone = data.clone();
    let parsed = match serde_json::from_value::<AccountCreateRequest>(data) {
        Ok(p) => p,
        Err(err) => {
            println!("AccountCreate: Invalid payload: {err} | Raw: {data_clone}");
            let _ = socket.emit("CreationResponse", "Invalid request data");
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

    if !SERVER_CHARACTER_NAME_REGEX.is_match(&parsed.name) {
        println!("AccountCreate: Invalid Name: {}", parsed.name);
        let _ = socket.emit("CreationResponse", "Invalid character name");
        return;
    }

    if let Some(email) = &parsed.email {
        if !email.is_empty() && !is_valid_mail(email) {
            println!("AccountCreate: Invalid Email: {:?}", parsed.email);
            let _ = socket.emit("CreationResponse", "Invalid email address");
            return;
        }
    }

    // FIXME: this looks silly
    let account_name = parsed.account_name;
    let name = parsed.name;
    let password = parsed.password;
    let email = parsed.email;

    if !check_creation_ratelimits(client_ip, &state, &config).await {
        let _ = socket.emit("CreationResponse", "New accounts per day exceeded");
        return;
    }

    let users: mongodb::Collection<Account> = state.db.collection(&config.db_collection);
    let account: Result<Option<Account>, mongodb::error::Error> = users
        .find_one(doc! { "AccountName": account_name.to_uppercase() }, None)
        .await;
    match account {
        Ok(Some(_)) => {
            let _ = socket.emit("CreationResponse", "Account already exists");
            return;
        }
        Err(err) => {
            println!("MongoDB error while checking existing account: {err}");
            let _ = socket.emit("CreationResponse", "Server error");
            return;
        }
        Ok(None) => {
            // Fallthrough because everything's fine
        }
    }

    // Create a hashed password and saves it with the account info
    let hash = match bcrypt::hash(password.to_uppercase(), 10) {
        Ok(h) => h,
        Err(e) => {
            println!("Password hashing failed: {e}");
            let _ = socket.emit("CreationResponse", "Server error");
            return;
        }
    };
    let mut account: Account;
    {
        let mut next_member_number = state.next_member_number.write().await;
        account = Account {
            account_name: account_name.to_uppercase(),
            name,
            password: Some(hash),
            email: Some(email.unwrap().to_string()),
            member_number: *next_member_number,
            //Lovership: [],
            item_permission: 2,
            friend_list: HashSet::new(),
            white_list: HashSet::new(),
            black_list: HashSet::new(),
            money: 100,
            creation: SystemTime::now().get_timestamp_in_milliseconds(),
            last_login: SystemTime::now().get_timestamp_in_milliseconds(),
            environment: "PROD".to_string(), //account_get_environment(socket),
            ..Default::default()
        };
        match users.insert_one(&account, None).await {
            Ok(_) => {
                *next_member_number += 1;
            }
            Err(e) => {
                println!("Account insertion failed: {e}");
                let _ = socket.emit("CreationResponse", "Server error");
            }
        };
    }
    account.id = Some(socket.id.to_string());
    account.socket = Some(socket.clone());
    //AccountValidData(account);
    //Account.push(account);
    //OnLogin(socket);

    let _ = socket.emit(
        "CreationResponse",
        &json!({
            "ServerAnswer": "AccountCreated",
            "OnlineID": account.account_name.to_uppercase(),
            "MemberNumber": account.member_number,
        }),
    );
    account_send_server_info(socket, state).await;
    //AccountPurgeInfo(data);
}
async fn check_creation_ratelimits(
    client_ip: HttpExtension<ConnectInfo<SocketAddr>>,
    state: &Arc<State>,
    config: &AppConfig,
) -> bool {
    let current_ip = client_ip.ip().to_canonical();
    let current_time = SystemTime::now();
    let mut total_count: u32 = 0;
    let mut hour_count: u32 = 0;
    let one_hour_ago = current_time - Duration::from_secs(3600);
    // loop
    {
        let account_creation_ip = state.account_creation_ip.read().await;

        for ip in account_creation_ip.iter() {
            if ip.address == current_ip {
                total_count += 1;
                if ip.time >= one_hour_ago {
                    hour_count += 1;
                }
            }
        }
    }
    // Exits if we reached the limit
    if total_count >= config.max_ip_account_per_day || hour_count >= config.max_ip_account_per_hour
    {
        return false;
    }
    {
        // Keeps the IP in memory for the next run
        let mut account_creation_ip = state.account_creation_ip.write().await;
        account_creation_ip.push(AccountCreationIP {
            address: current_ip,
            time: current_time,
        });
    }
    true
}
