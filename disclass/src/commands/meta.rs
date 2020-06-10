use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Pong!")?;
    Ok(())
}

#[command]
pub fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    
    dbg!(&msg);
    
    msg.channel_id
        .say(&ctx.http, "Hello I am Heisenberg the bot")?;
    Ok(())
}
