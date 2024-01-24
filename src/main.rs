
// Importing the required modules from the standard library and Serenity crate.
use std::env;

use serenity::model::user::OnlineStatus;
use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::{Activity, Ready},
    },
    prelude::*,
};

// Importing local modules.
mod commands;
mod core_functions;
mod memory_gauge;
mod responses;

struct Handler;

// Implementing the EventHandler trait for the Handler struct.
#[async_trait]
impl EventHandler for Handler {
    // Asynchronous function to handle incoming messages.
    async fn message(&self, ctx: Context, msg: Message) {
        // Check if the author of the message is not a bot.
        if !msg.author.bot {
            // Call the command handling function from the commands module.
            unsafe {
                commands::command(ctx, msg).await;
            }
        }
    }
    // Asynchronous function to handle the bot being ready.
    async fn ready(&self, ctx: Context, ready: Ready) {
        // Print a message to the console indicating the bot is connected.
        println!("{} is connected!", ready.user.name);
        // Set the bot's presence to "Watching 0 games." when ready.
        let activity = Activity::watching("0 games.");
        let status = OnlineStatus::Online;
        ctx.set_presence(Some(activity), status).await;
    }
}

// Main function to start the bot.
#[tokio::main]
async fn main() {
    // Retrieve the bot token from the environment.
    let token = env::var("LUNABOT_TOKEN").expect("Expected a token in the environment");

    // Define the bot's intents to receive relevant events.
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new Serenity client with the specified token, intents, and event handler.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Start the bot, handling any errors that may occur.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
