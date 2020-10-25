use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::channel::Message,
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group
        }
    },
};

use std::env;

mod handler;
mod pg;

#[tokio::main]
async fn main() {
    let token = env::var("ZENCHI_DISCORD_BOT_TOKEN").expect("set 'ZENCHI_DISCORD_BOT_TOKEN' to environment variable");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("z/"))
        .group(&handler::SEQUENCE_GROUP);
    
    let mut client = Client::builder(token)
        .event_handler(handler::Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("failed to start client. cause : {}", why);
    }
}