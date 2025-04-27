use std::env;
use dotenv::dotenv;

use serenity::all::{Message, Reaction, Ready};
use serenity::async_trait;
use serenity::prelude::*;

mod role_assignment
{
    pub mod reaction_handler;
}

mod commands{
    pub mod text_commands;
}

use commands::text_commands::TextCommands;
use role_assignment::reaction_handler::ReactionHandler;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        TextCommands::message( ctx, msg ).await;
    }
    
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        ReactionHandler::reaction_add(ctx, reaction).await;
    }
    
    async fn  reaction_remove(&self, ctx: Context, reaction: Reaction) {
        ReactionHandler::reaction_remove(ctx, reaction).await;
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