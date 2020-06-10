use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::{Reaction, ReactionType};
use serenity::model::{event::ResumedEvent, gateway::Ready};

use std::env;
mod commands;

use commands::{meta::*, quizz::*};

#[group]
#[commands(ping, help, quizz)]
struct General;

struct Handler;

impl EventHandler for Handler {
    // says a message whenever a reaction is made
    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        dbg!(&reaction);

        if let Err(why) = reaction.channel_id.say(
            &ctx.http,
            format!(
                "{} left a {} reaction",
                reaction.user(&ctx).unwrap().name,
                match reaction.emoji {
                    ReactionType::Custom {
                        animated: _animated,
                        id: _id,
                        name,
                    } => name.unwrap(),
                    ReactionType::Unicode(uni) => uni,
                    ReactionType::__Nonexhaustive => String::new(),
                }
            ),
        ) {
            println!("Error reacting to a reaction: {:?}", why);
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

fn main() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").unwrap(), Handler).unwrap();

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("$"))
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
