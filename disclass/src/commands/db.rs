use std::env;
use mongodb::{
    bson::{doc, Bson},
    sync::{Client, Database},
    error::Error
};


pub fn get_database() -> Result<Database, Error> {
    let db_uri = env::var("MONGODB_URI").unwrap_or("mongodb://localhost:27017".to_string());
    let client = Client::with_uri_str(&db_uri)?;
    Ok(client.database(&env::var("MONGODB_DB").unwrap_or("disclass".to_string())))
}


// fn example() {
//     let client = Client::with_uri_str("mongodb://localhost:27017")?;
//     let database = client.database("mydb");
//     let collection = database.collection("books");
    
//     let docs = vec![
//         doc! { "title": "1984", "author": "George Orwell" },
//         doc! { "title": "Animal Farm", "author": "George Orwell" },
//         doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
//     ];
    
//     // Insert some documents into the "mydb.books" collection.
//     collection.insert_many(docs, None)?;
    
//     let cursor = collection.find(doc! { "author": "George Orwell" }, None)?;
//     for result in cursor {
//         match result {
//             Ok(document) => {
//                 if let Some(title) = document.get("title").and_then(Bson::as_str) {
//                     println!("title: {}", title);
//                 } else {
//                     println!("no title found");
//                 }
//             }
//             Err(e) => return Err(e.into()),
//         }
//     }
// }
