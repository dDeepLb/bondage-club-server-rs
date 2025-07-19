use crate::{
    common::{
        protocol::{
            AccountBeepRequest, AccountCreateRequest, AccountLoginRequest, AccountQueryRequest,
            AccountUpdateRequest,
        },
        types::{AccountCreationIP, LoginQueueStruct},
    },
    models::account::Account,
};
use axum::extract::ConnectInfo;
use dotenvy::dotenv;
use figment::{Figment, providers::Env};
use futures_util::stream::StreamExt;
use mongodb::bson::{Document, doc};
use mongodb::{Collection, Database};
use ordermap::{OrderMap, OrderSet};
use serde::Deserialize;
use socketioxide::{
    SocketIo,
    extract::{Data, HttpExtension, SocketRef},
    socket::Sid,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::{Mutex, RwLock};

pub struct BCServer {
    pub config: AppConfig,
    pub db: Database,
    pub accounts: Mutex<Vec<Account>>,
    pub next_member_number: RwLock<u32>,
    pub account_creation_ip: RwLock<Vec<AccountCreationIP>>,
    pub login_queue: RwLock<OrderMap<Sid, LoginQueueStruct>>,
    pub pending_logins: RwLock<OrderSet<Sid>>,
    pub io: SocketIo,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server_addr: String,
    pub db_uri: String,
    pub db_name: String,
    pub db_accounts: String,
    pub max_ip_account_per_day: u32,
    pub max_ip_account_per_hour: u32,
}

pub fn load_config() -> AppConfig {
    dotenv().ok();

    Figment::from(Env::prefixed("APP_"))
        .extract()
        .expect("Failed to load configuration")
}

impl BCServer {
    pub async fn new(db: Database, io: SocketIo) -> Arc<Self> {
        let config = load_config();
        match db.run_command(doc! { "ping": 1 }, None).await {
            Ok(_) => {
                println!("****************************************");
                println!("Database: {} connected", config.db_name);
                println!("****************************************");
            }
            Err(e) => {
                eprintln!("❌ Failed to ping MongoDB: {e}");
                panic!("Cannot continue without database connection");
            }
        }

        // Find the highest MemberNumber
        let accounts: Collection<Document> = db.collection(&config.db_accounts);
        let filter = doc! { "MemberNumber": { "$exists": true, "$ne": null } };
        let find_options = mongodb::options::FindOptions::builder()
            .sort(doc! { "MemberNumber": -1 })
            .limit(1)
            .build();

        let mut cursor = accounts.find(filter, find_options).await.unwrap();
        let mut next_member_number = 1;

        if let Some(Ok(doc)) = cursor.next().await {
            if let Ok(member_num) = doc.get_i32("MemberNumber") {
                next_member_number = (member_num + 1) as u32;
            }
        }

        println!("Next Member Number: {next_member_number}");

        let server = Arc::new(Self {
            db,
            config,
            accounts: Mutex::new(<Vec<Account>>::new()),
            next_member_number: RwLock::new(next_member_number),
            account_creation_ip: RwLock::new(<Vec<AccountCreationIP>>::new()),
            login_queue: RwLock::new(OrderMap::new()),
            pending_logins: RwLock::new(OrderSet::new()),
            io,
        });

        server.clone().register_handlers();

        server
    }

    pub fn register_handlers(self: Arc<Self>) {
        let io = self.io.clone();
        io.ns(
            "/",
            move |socket: SocketRef, client_ip: HttpExtension<ConnectInfo<SocketAddr>>| {
                self.on_connect(socket, client_ip);
            },
        );
    }

    fn on_connect(
        self: Arc<Self>,
        socket: SocketRef,
        client_ip: HttpExtension<ConnectInfo<SocketAddr>>,
    ) {
        let ip = client_ip.ip();
        let port = client_ip.port();
        let _socket_id = socket.id;

        println!("Connected: {}, {ip}:{port}", socket.id);
        let id = socket.id;
        socket.on_disconnect(move || {
            println!("Disconnected: {id}, {ip}:{port}");
        });

        let server = self.clone();
        socket.on(
            "AccountCreate",
            move |socket: SocketRef, Data(data): Data<serde_json::Value>, client_ip| {
                let server = server.clone();
                Box::pin(async move {
                    let data_clone = data.clone();
                    match serde_json::from_value::<AccountCreateRequest>(data) {
                        Ok(req) => {
                            server.on_account_create(socket, req, client_ip).await;
                        }
                        Err(err) => {
                            println!("AccountCreate: Invalid payload: {err} | Raw: {data_clone}");
                            let _ = socket.emit("CreationResponse", "Invalid request data");
                        }
                    }
                })
            },
        );

        let server = self.clone();
        socket.on(
            "AccountLogin",
            move |socket: SocketRef, Data(data): Data<serde_json::Value>| {
                let server = server.clone();
                Box::pin(async move {
                    let data_clone = data.clone();
                    match serde_json::from_value::<AccountLoginRequest>(data) {
                        Ok(req) => {
                            server.on_account_login(socket, req).await;
                        }
                        Err(err) => {
                            println!("AccounLogin: Invalid payload: {err} | Raw: {data_clone}");
                            let _ = socket.emit("LoginResponse", "Invalid request data");
                        }
                    }
                })
            },
        );

        let server = self.clone();
        socket.on(
            "AccountUpdate",
            move |socket: SocketRef, Data(data): Data<serde_json::Value>| {
                let server = server.clone();
                Box::pin(async move {
                    let data_clone = data.clone();
                    match serde_json::from_value::<AccountUpdateRequest>(data) {
                        Ok(req) => {
                            println!("AccountUpdate: received {req:?}");
                            server.on_account_update(socket, req).await;
                        }
                        Err(err) => {
                            println!("AccountUpdate: Invalid payload: {err} | Raw: {data_clone}");
                            let _ = socket.emit("LoginResponse", "Invalid request data");
                        }
                    }
                })
            },
        );

        let server = self.clone();
        socket.on(
            "AccountBeep",
            move |socket: SocketRef, Data(data): Data<serde_json::Value>| {
                let server = server.clone();
                Box::pin(async move {
                    let data_clone = data.clone();
                    match serde_json::from_value::<AccountBeepRequest>(data) {
                        Ok(req) => {
                            server.on_account_beep(socket, req).await;
                        }
                        Err(err) => {
                            println!("AccountBeep: Invalid payload: {err} | Raw: {data_clone}");
                            let _ = socket.emit("LoginResponse", "Invalid request data");
                        }
                    }
                })
            },
        );

        let server = self.clone();
        socket.on(
            "AccountQuery",
            move |socket: SocketRef, Data(data): Data<serde_json::Value>| {
                let server = server.clone();
                Box::pin(async move {
                    let data_clone = data.clone();
                    match serde_json::from_value::<AccountQueryRequest>(data) {
                        Ok(req) => {
                            server.on_account_query(socket, req).await;
                        }
                        Err(err) => {
                            println!("AccountQuery: Invalid payload: {err} | Raw: {data_clone}");
                            let _ = socket.emit("LoginResponse", "Invalid request data");
                        }
                    }
                })
            },
        );
    }
}
