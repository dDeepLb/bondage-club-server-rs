use async_recursion::async_recursion;
use mongodb::bson::{Bson, doc};
use socketioxide::extract::SocketRef;
use std::time::SystemTime;

use crate::{
    common::{
        constants::{SERVER_ACCOUNT_NAME_REGEX, SERVER_ACCOUNT_PASSWORD_REGEX},
        protocol::AccountLoginRequest,
        types::LoginQueueStruct,
    },
    models::account::{self, Account},
    server::BCServer,
    utilities::{
        millis_timestamps::SystemTimeMillisTimestamps,
        socket_account::{attach_account_to_socket, get_account_from_member_number},
    },
};

impl BCServer {
    pub async fn on_account_login(&self, socket: SocketRef, request: AccountLoginRequest) {
        if !SERVER_ACCOUNT_NAME_REGEX.is_match(&request.account_name) {
            println!(
                "AccountCreate: Invalid AccountName: {}",
                request.account_name
            );
            let _ = socket.emit("CreationResponse", "Invalid account name");
            return;
        }

        if !SERVER_ACCOUNT_PASSWORD_REGEX.is_match(&request.password) {
            println!("AccountCreate: Invalid Password");
            let _ = socket.emit("CreationResponse", "Invalid password");
            return;
        }

        let uppercase_account_name = request.account_name.to_uppercase();
        let should_run;
        {
            let mut pending_logins = self.pending_logins.write().await;
            let mut login_queue = self.login_queue.write().await;
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
                    password: request.clone().password,
                },
            );
            pending_logins.insert(socket.id);

            if login_queue.len() > 16 {
                let _ = socket.emit("LoginQueue", &login_queue.len());
            }
        }
        // If there are no logins being processed, start the processing of the queue
        if should_run {
            self.account_login_run().await;
        }
    }

    #[async_recursion]
    async fn account_login_run(&self) {
        {
            let mut login_queue = self.login_queue.write().await;
            let mut pending_logins = self.pending_logins.write().await;
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
            self.account_login_process(next.socket.clone(), next.account_name, next.password)
                .await;

            pending_logins.remove(&next.socket.id);
            login_queue.remove(&next.socket.id);

            if !login_queue.is_empty() {
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                self.account_login_run().await;
            }
        }
    }

    async fn account_login_process(
        &self,
        socket: SocketRef,
        account_name: String,
        password: String,
    ) {
        let users: mongodb::Collection<Account> = self.db.collection(&self.config.db_accounts);

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
            let account =
                get_account_from_member_number(&self.io, account_result.member_number).unwrap();
            let account = account.lock().unwrap();
            if account.socket.is_some() {
                let socket = account.socket.as_ref().unwrap();
                let _ = socket.emit("ForceDisconnect", "ErrorDuplicatedLogin");
                let _ = <socketioxide::extract::SocketRef as Clone>::clone(socket).disconnect();
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
            attach_account_to_socket(&socket, account_result.clone()).await;
        }
        //OnLogin(socket);
        let _ = socket.emit("LoginResponse", &account_result);

        /* 	/** @type {Account|null} */
           Account.push(result);
           OnLogin(socket);
           delete result.Password;
           delete result.Email;
           socket.compress(false).emit("LoginResponse", result);
           result.Socket = socket;
           AccountSendServerInfo(socket);
           AccountPurgeInfo(result);
        */
    }
}
