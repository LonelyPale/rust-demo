use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    options::FindOptions,
    Client,
};
use serde::{Deserialize, Serialize};

async fn get_client() -> mongodb::error::Result<Client> {
    let uri = "<connection string>";
    let uri = "mongodb://root:mongorsqwer1234@124.221.220.108:51040,124.221.220.108:52040,124.221.220.108:53040/admin?replicaSet=rs";
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.app_name = Some("Demo App".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[tokio::test]
async fn info() -> mongodb::error::Result<()> {
    let client = get_client().await?;

    for db_name in client.list_database_names(None, None).await? {
        println!("db: {}", db_name);
        let db = client.database(&db_name);
        for collection_name in db.list_collection_names(None).await? {
            println!("\tcoll: {}", collection_name);
        }
    }

    Ok(())
}

#[tokio::test]
async fn insert_many() -> mongodb::error::Result<()> {
    let client = get_client().await?;
    let db = client.database("test");
    let coll = db.collection::<Document>("test");

    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    let result = coll.insert_many(docs, None).await?;
    println!("insert_many: {:?}", result);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

#[tokio::test]
async fn insert_many2() -> mongodb::error::Result<()> {
    let client = get_client().await?;
    let db = client.database("test");
    let coll = db.collection::<Book>("test");

    let books = vec![
        Book {
            title: "The Grapes of Wrath".to_string(),
            author: "John Steinbeck".to_string(),
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
        },
    ];

    let result = coll.insert_many(books, None).await?;
    println!("insert_many: {:?}", result);

    Ok(())
}

#[tokio::test]
async fn find() -> mongodb::error::Result<()> {
    let client = get_client().await?;
    let db = client.database("test");
    let coll = db.collection::<Book>("test");

    let filter = doc! { "author": "George Orwell" };
    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = coll.find(filter, find_options).await?;

    while let Some(book) = cursor.try_next().await? {
        println!("title: {}\t\tauthor: {}", book.title, book.author);
    }

    Ok(())
}
