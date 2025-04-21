use std::env;
use dotenv::dotenv;

use serenity::all::{Message, Reaction, Ready};

use serenity::async_trait;
use serenity::prelude::*;

struct Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping"
        {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await
            {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        println!("Handling reaction: {:?}", reaction);

        if reaction.message_id == 1363815289522749514
        {
            let emoji_str = reaction.emoji.unicode_eq("👍");

            if emoji_str == true
            {
                let _ = reaction.channel_id.say(&ctx.http, "DIL IS GAY").await;
            }
        }

}

async fn ready(&self, _ctx: Context, ready: Ready)
    {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main()
{
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT
    | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}