use std::env;
use serenity::{
    model::{
        channel::{
            Message, 
            Reaction, 
            ReactionType
        },
        gateway::{
            Activity,
            ActivityEmoji,
            Ready
        }, 
        event::ResumedEvent,
        user::OnlineStatus,
    },
    prelude::*,
};


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


    fn message(&self, context: Context, msg: Message) {

        dbg!(&msg);

        if msg.content.starts_with("$show") {
            let term = msg.content.split_ascii_whitespace().last().unwrap();
            
            let emoji = match term.to_ascii_lowercase().as_ref() {
                "avocado" => "🥑",
                "crab" =>  "🦀",
                "snake" =>  "🐍",
                _ => "❌",
            };

            if let Err(reason) = msg.channel_id.say(&context.http, &emoji) {
                println!("Error happened: {:?}", reason);
            }

        }
        
        if msg.content == "!message" {

            let dm = msg.author.dm(&context, |m| {
                m.content("Hello!");
                let myreactions = vec!["1️⃣", "2️⃣", "3️⃣", "4️⃣"];
                m.reactions(myreactions);
                m
            });

            match dm {
                Ok(_) => {
                    let _ = msg.react(&context, "✅");
                },
                Err(why) => {
                    println!("Err sending help: {:?}", why);

                    let _ = msg.reply(&context, "There was an error DMing you help.");
                },
            };
            // if let Err(why) = dm {
            //     println!("Error when direct messaging user: {:?}", why);
            // }
        }
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        let mut activity = Activity::playing("🎓 $help for help");
        activity.emoji = Some(ActivityEmoji{name: "🎓".to_string(), id: None, animated: None});
        let status = OnlineStatus::Online;

        ctx.set_presence(Some(activity), status);
        println!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler)
        .expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
