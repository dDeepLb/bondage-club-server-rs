use std::sync::Arc;

use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use socketioxide::extract::{Data, SocketRef};

use crate::common::config::{
    self,
    types::{Account, ServerFriendInfo, State},
};
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct AccountQueryRequest {
    query: String,
}

pub async fn account_query(Data(data): Data<Value>, socket: SocketRef, state: Arc<State>) {
    let data_clone = data.clone();
    let parsed = match serde_json::from_value::<AccountQueryRequest>(data) {
        Ok(p) => p,
        Err(err) => {
            println!("AccountQuery: Invalid payload: {err} | Raw: {data_clone}");
            let _ = socket.emit("AccountQueryResult", "Invalid request data");
            return;
        }
    };

    // Finds the current account
    let accounts: tokio::sync::RwLockReadGuard<'_, Vec<Account>> = state.accounts.read().await;
    let player = accounts
        .iter()
        .find(|a| a.id == Some(socket.id.to_string()));
    if player.is_none() {
        return;
    }
    let player = player.unwrap();
    // OnlineFriends query - returns all friends that are online and the room name they are in
    if parsed.query == "OnlineFriends" {
        // Add all submissives owned by the player and all lovers of the players to the list
        let mut friends = vec![];
        for account in accounts.iter() {
            let is_owned = account.ownership.is_some()
                && account.ownership.as_ref().unwrap().member_number == player.member_number;

            /*  let is_lover = account
            .lovership
            .iter()
            .find(|l| l.member_number == player.member_number); */
            if is_owned
            /* || is_lover */
            {
                friends.push(ServerFriendInfo {
                    r#type: if is_owned { "Submissive" } else { "Lover" },
                    member_number: account.member_number,
                    member_name: account.name.clone(),
                    // todo: Chatroom
                    /*  chat_room_space: if friend.chat_room.is_some() {
                        friend.chat_room.as_ref().unwrap().space
                    } else {
                        None
                    },
                    chat_room_name: if friend.chat_room.is_some() {
                        friend.chat_room.as_ref().unwrap().name
                    } else {
                        None
                    },
                    private: if friend.chat_room.is_some()
                        && friend.chat_room.as_ref().unwrap().private
                    {
                        true
                    } else {
                        None
                    }, */
                });
            }
            if account.friend_list.contains(&player.member_number)
                && player.friend_list.contains(&account.member_number)
            {
                friends.push(ServerFriendInfo {
                    r#type: "Friend",
                    member_number: account.member_number,
                    member_name: account.name.clone(),
                    // todo: Chatroom
                    /*  chat_room_space: if friend.chat_room.is_some() {
                        friend.chat_room.as_ref().unwrap().space
                    } else {
                        None
                    },
                    chat_room_name: if friend.chat_room.is_some() {
                        friend.chat_room.as_ref().unwrap().name
                    } else {
                        None
                    },
                    private: if friend.chat_room.is_some()
                        && friend.chat_room.as_ref().unwrap().private
                    {
                        true
                    } else {
                        None
                    }, */
                });
            }
        }
        let _ = socket.emit(
            "AccountQueryResult",
            &json!({ "Query": parsed.query, "Result": friends }),
        );
    }

    if parsed.query == "EmailStatus" {
        let config = config::load_config();
        let users: mongodb::Collection<Account> = state.db.collection(&config.db_collection);
        let account_result = users
            .find_one(doc! { "AccountName": player.account_name.clone(), "mail": { "$exists": true, "$ne": "" }  }, None)
            .await;
        match account_result {
            Ok(Some(_account)) => {
                let _ = socket.emit(
                    "AccountQueryResult",
                    &json!({ "Query": parsed.query, "Result": true }),
                );
            }
            Ok(None) => {
                let _ = socket.emit(
                    "AccountQueryResult",
                    &json!({ "Query": parsed.query, "Result": false }),
                );
            }
            Err(err) => {
                println!("MongoDB error while checking existing account: {err}");
                let _ = socket.emit("AccountQueryResult", "Server error");
            }
        }
    }
}
