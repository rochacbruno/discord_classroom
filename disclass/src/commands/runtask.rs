use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::model::guild::GuildContainer;

use mongodb::{
    bson::{doc, bson, Bson},
    sync::{Client, Database},
    error::Error
};

use crate::commands::db::get_database;


#[command]
pub fn runtask(ctx: &mut Context, msg: &Message) -> CommandResult {
    dbg!(&msg);
    let admin_role = 722653585506041867u64;  // TODO: Search for role named `admin` 

    if let Some(guild_id) = msg.guild_id {
        let is_admin = &msg.author.has_role(&ctx, guild_id, admin_role);
        
        if let Some(attachment) = msg.attachments.first() {
            let file_content = attachment.download();
            // TODO: Process the YAML File.
        }
    
    
    } else {
        let dm = msg.author.dm(&ctx, |m| {
            m.content(
                "❌ You are not authorized to run this task ❌"
            );
            m
        });
    }
    
    // Assign new quizz to the user (latest)
    // let db = get_database()?;
    // let quizz = db.collection("quizz");
    // let quizz_request = db.collection("quizz_request");

    // quizz_request.insert_one(doc!{ "quizz": "0000", "user": msg.author.id.to_string()}, None)?;
    

    Ok(())
}
