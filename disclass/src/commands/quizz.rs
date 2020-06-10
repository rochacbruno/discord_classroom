use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;



#[command]
pub fn quizz(ctx: &mut Context, msg: &Message) -> CommandResult {
    // Verify if user has running quizzes
    // let user = User::from_userid(msg.author.id)?;
    
    // Assign new quizz to the user (latest)
    

    let dm = msg.author.dm(&ctx, |m| {
        m.content("Hello!");
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
