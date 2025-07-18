use futures_util::stream::StreamExt;
use mongodb::{
    Client, Collection,
    bson::{Document, doc},
    options::ClientOptions,
};
use ordermap::{OrderMap, OrderSet};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::common::config::types::{Account, AccountCreationIP, State};

pub async fn setup_mongodb(uri: &str, db_name: &str, collection_name: &str) -> Arc<State> {
    let options = ClientOptions::parse(uri).await.unwrap();
    let client = Client::with_options(options).unwrap();
    let db = client.database(db_name);

    match db.run_command(doc! { "ping": 1 }, None).await {
        Ok(_) => {
            println!("****************************************");
            println!("Database: {db_name} connected");
            println!("****************************************");
        }
        Err(e) => {
            eprintln!("❌ Failed to ping MongoDB: {e}");
            panic!("Cannot continue without database connection");
        }
    }

    // Find the highest MemberNumber
    let collection: Collection<Document> = db.collection(collection_name);
    let filter = doc! { "MemberNumber": { "$exists": true, "$ne": null } };
    let find_options = mongodb::options::FindOptions::builder()
        .sort(doc! { "MemberNumber": -1 })
        .limit(1)
        .build();

    let mut cursor = collection.find(filter, find_options).await.unwrap();
    let mut next_member_number = 1;

    if let Some(Ok(doc)) = cursor.next().await {
        if let Ok(member_num) = doc.get_i32("MemberNumber") {
            next_member_number = (member_num + 1) as u32;
        }
    }

    println!("Next Member Number: {next_member_number}");

    Arc::new(State {
        db,
        next_member_number: RwLock::new(next_member_number),
        account_creation_ip: RwLock::new(<Vec<AccountCreationIP>>::new()),
        accounts: RwLock::new(<Vec<Account>>::new()),
        login_queue: RwLock::new(OrderMap::new()),
        pending_logins: RwLock::new(OrderSet::new()),
    })
}
