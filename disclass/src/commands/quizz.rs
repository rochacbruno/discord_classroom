use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use mongodb::{
    bson::{doc, bson, Bson},
    sync::{Client, Database},
    error::Error
};

use crate::commands::db::get_database;


#[command]
pub fn quizz(ctx: &mut Context, msg: &Message) -> CommandResult {
    // Verify if user has running quizzes
    // let user = User::from_userid(msg.author.id)?;
    
    // Assign new quizz to the user (latest)
    let db = get_database()?;
    let quizz = db.collection("quizz");
    let quizz_request = db.collection("quizz_request");

    quizz_request.insert_one(doc!{ "quizz": "0000", "user": msg.author.id.to_string()}, None)?;
    dbg!(msg);
    

    let dm = msg.author.dm(&ctx, |m| {
        m.content(
            "Hello! `def helllo():\n print('Hello')\n` "
        );
        let myreactions = vec!["1️⃣", "2️⃣", "3️⃣", "4️⃣"];
        m.reactions(myreactions);
        m
    });

    match dm {
        Ok(_) => {
            let _ = msg.react(&ctx, "✅");
        }
        Err(why) => {
            println!("Err sending help: {:?}", why);

            let _ = msg.reply(&ctx, "There was an error DMing you help.");
        }
    };

    Ok(())
}
