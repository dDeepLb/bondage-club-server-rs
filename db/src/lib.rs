use mongodb::{Client, Collection, Database, options::ClientOptions, bson::{doc, Document}};
use std::sync::Arc;
use tokio::sync::RwLock;
use futures_util::stream::StreamExt;

pub struct State {
    pub db: Database,
    pub next_member_number: RwLock<u32>,
}

pub async fn setup_mongodb(uri: &str, db_name: &str, collection_name: &str) -> Arc<State> {
    let options = ClientOptions::parse(uri).await.unwrap();
    let client = Client::with_options(options).unwrap();
    let db = client.database(db_name);

    println!("****************************************");
    println!("Database: {db_name} connected");
    println!("****************************************");

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
    })
}