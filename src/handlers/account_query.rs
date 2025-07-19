use mongodb::bson::doc;
use serde_json::json;
use socketioxide::extract::SocketRef;

use crate::{
    common::protocol::AccountQueryRequest,
    models::account::{Account, ServerFriendInfo},
    server::BCServer,
};

impl BCServer {
    pub async fn on_account_query(&self, socket: SocketRef, request: AccountQueryRequest) {
        // Finds the current account
        let accounts = self.accounts.lock().await;
        let player = accounts
            .iter()
            .find(|a| a.id == Some(socket.id.to_string()));
        if player.is_none() {
            return;
        }
        let player = player.unwrap();
        // OnlineFriends query - returns all friends that are online and the room name they are in
        if request.query == "OnlineFriends" {
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
                &json!({ "Query": request.query, "Result": friends }),
            );
        }

        if request.query == "EmailStatus" {
            let accounts: mongodb::Collection<Account> =
                self.db.collection(&self.config.db_accounts);
            let account_result = accounts
                .find_one(
                    doc! {
                        "AccountName": player.account_name.clone(),
                        "mail": { "$exists": true, "$ne": "" }
                    },
                    None,
                )
                .await;
            match account_result {
                Ok(Some(_account)) => {
                    let _ = socket.emit(
                        "AccountQueryResult",
                        &json!({ "Query": request.query, "Result": true }),
                    );
                }
                Ok(None) => {
                    let _ = socket.emit(
                        "AccountQueryResult",
                        &json!({ "Query": request.query, "Result": false }),
                    );
                }
                Err(err) => {
                    println!("MongoDB error while checking existing account: {err}");
                    let _ = socket.emit("AccountQueryResult", "Server error");
                }
            }
        }
    }
}
