// Importing required modules from the standard library and Serenity crate.

use std::env;

use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::channel::{AttachmentType, Message};
use serenity::model::gateway::Activity;
use serenity::model::prelude::OnlineStatus;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// Importing modules from the local crate.
use crate::{commands, memory_gauge, responses};

// Function to extract the author's ID from a message.
pub fn author(msg: Message) -> i64 {
    msg.author.id.to_string().clone().parse::<i64>().unwrap()
}

// Asynchronous function to change the bot's activity.
pub async unsafe fn change_activity(ctx: Context) {
    // Getting the count of games from the commands module.
    let games = commands::GAME_COUNT;
    let mut plural_checker = " games.";
    // Checking if there's only one game to adjust grammar.
    if games == 1 {
        plural_checker = " game.";
    }
    // Creating a 'watching' activity with the game count.
    let activity = Activity::watching(games.to_string() + plural_checker);
    let status = OnlineStatus::Online;
    // Setting the bot's presence with the new activity.
    ctx.set_presence(Some(activity), status).await;
}

// Asynchronous function to convert a user ID to a user tag.
pub async fn to_tag(user_id: u64) -> String {
    // Retrieving the bot token from the environment.
    let token = env::var("LUNABOT_TOKEN").expect("Expected a token in the environment");
    let http = serenity::http::client::Http::new(token.as_str());
    // Fetching the user information using the Serenity HTTP client.
    let user = http.get_user(user_id).await.expect("TODO: panic message");
    // Extracting the user tag and removing the discriminator.
    user.tag().replace("#0000", "")
}

// Asynchronous function to check if a user is in a game.
pub async unsafe fn in_game(msg: Message, arguments: &str) -> i64 {
    let mut response = 0;
    let mut mentioned_id = 0;

    let author_id = author(msg.clone());

    // Checking if the message contains mentions.
    if !msg.mentions.is_empty() {
        mentioned_id = msg.mentions.first().unwrap().id.to_string().parse::<i64>().unwrap();
    }
    if arguments.is_empty() {
        response = 0;
    } else if mentioned_id == author_id {
        response = 1;
    } else if mentioned_id != 0 {
        // Checking if the mentioned user is a bot or part of a game.
        if msg.mentions.first().unwrap().bot {
            response = 2;
        } else {
            // Iterating through the game data to check if users are part of a game.
            let mut found_in_game = false;
            let mut i = 0;
            while i < commands::GAME_DATA.len() {
                let test1 = *commands::GAME_DATA.first().unwrap().first().unwrap();
                let test2 = *commands::GAME_DATA.first().unwrap().get(1).unwrap();
                println!("{}", test1.to_string() + " " + &test2.to_string().as_mut_str());
                if test1 == author_id || test1 == mentioned_id || test2 == author_id || test2 == mentioned_id {
                    found_in_game = true;
                }
                i += 1;
            }
            if found_in_game {
                response = 3;
            } else if !found_in_game {
                response = 4;
            }
        }
    } else {
        response = 5;
    }
    response
}

// Asynchronous function to add a new game.
pub async unsafe fn add_game(ctx: Context, msg: Message, energy: i64, turn: i64, board: i64, board2: i64, counter: i64, color: i64, color2: i64) {
    let player_one = author(msg.clone());
    let mut player_two = 0;
    // Checking if the message contains mentions for a second player.
    if !msg.mentions.is_empty() {
        player_two = msg.mentions.first().unwrap().id.to_string().parse::<i64>().unwrap();
    }
    // Creating a new game and updating game count and bot's activity.
    make_game(ctx.clone(), player_one, player_two, energy, turn, board, board2, counter, color, color2).await;

    let game = find_game(msg.clone()).await;
    let player_two = read_game(game, 1).await;
    let player_one = to_tag(player_one as u64).await;
    let player_two = to_tag(player_two as u64).await;
    let response = responses::GAME_START.replace("PLAYER_ONE", player_one.as_str()).replace("PLAYER_TWO", player_two.as_str());
    send_game(ctx.clone(), msg.clone(), game, table_id(msg).await as i32, response.as_str(), "Game Start").await;
    commands::GAME_COUNT += 1;
    change_activity(ctx).await;
}

// Asynchronous function to end a game.
pub async unsafe fn end_game(table: usize) {
    // Removing the game data for the specified table.
    commands::GAME_DATA.remove(table);
}

// Asynchronous function to find a game based on a message.
pub async unsafe fn find_game(msg: Message) -> [i64; 9] {
    let table: usize = table_id(msg).await;
    let game = *commands::GAME_DATA.get(table).unwrap();
    game
}

// Asynchronous function to read a specific value from a game.
pub async unsafe fn read_game(game: [i64; 9], num: usize) -> i64 {
    *game.get(num).unwrap()
}

// Asynchronous function to get the table ID for a user in a game.
pub async unsafe fn table_id(msg: Message) -> usize {
    let player_one = author(msg.clone());
    let mut player_two = 0;
    let mut second_test = false;
    if !msg.mentions.is_empty() {
        player_two = msg.mentions.first().unwrap().id.0 as i64;
        second_test = true;
    }
    let mut i = 0;
    let mut table = usize::MAX;
    // Iterating through game data to find the table ID.
    while i < commands::GAME_DATA.len() {
        let current_game = commands::GAME_DATA.get(i);
        let test1 = *current_game.unwrap().first().unwrap();
        let test2 = *current_game.unwrap().get(1).unwrap();
        if test1 == player_one || test2 == player_one {
            table = i;
            i = commands::GAME_DATA.len() - 1;
        }
        if second_test && (test1 == player_two || test2 == player_two) {
            table = i;
            i = commands::GAME_DATA.len() - 1;
        }
        i += 1;
    }
    table
}

// Asynchronous function to change a specific value in a game.
pub async unsafe fn change_value(ctx: Context, msg: Message, location: i64, value: i64) {
    let table = table_id(msg.clone()).await;
    let game = find_game(msg).await;
    let mut player_one = read_game(game, 0).await;
    let mut player_two = read_game(game, 1).await;
    let mut energy = read_game(game, 2).await;
    let mut turn = read_game(game, 3).await;
    let mut board = read_game(game, 4).await;
    let mut board2 = read_game(game, 5).await;
    let mut counter = read_game(game, 6).await;
    let mut color = read_game(game, 7).await;
    let mut color2 = read_game(game, 8).await;

    // Changing the specified value in the game data.
    match location {
        0 => player_one = value,
        1 => player_two = value,
        2 => energy = value,
        3 => turn = value,
        4 => board = value,
        5 => board2 = value,
        6 => counter = value,
        7 => color = value,
        8 => color2 = value,
        _ => {}
    }
    // Ending the current game and creating a new game with updated values.
    end_game(table).await;
    make_game(ctx, player_one, player_two, energy, turn, board, board2, counter, color, color2).await;
}

// Asynchronous function to create a new game and add it to the game data.
pub async unsafe fn make_game(ctx: Context, player_one: i64, player_two: i64, energy: i64, turn: i64, board: i64, board2: i64, counter: i64, color: i64, color2: i64) {
    commands::GAME_DATA.push([player_one, player_two, energy, turn, board, board2, counter, color, color2]);
    // Updating the bot's activity with the new game count.
    change_activity(ctx).await;
}

// Asynchronous function to send the game state as an image.
pub async unsafe fn send_game(ctx: Context, msg: Message, game: [i64; 9], table: i32, desc: &str, title: &str) {
    // Sending the game state as an image using the memory_gauge module.
    send_embed_image(ctx, msg, title, desc, memory_gauge::create_gauge(game, table).await.as_str()).await;
}

pub async fn send_help(ctx: Context, msg: Message) {
    // Create a mutable embed builder
    let mut embed = CreateEmbed::default();
    // Set embed properties
    embed = embed.author(|a| a.name("Lunabot").url(responses::INVITE_LINK).icon_url("https://dema.pink/lunabot.png")).to_owned();
    embed = embed.title("Lunamon Info").to_owned();
    embed = embed.description(responses::HELP_MESSAGE).to_owned();
    embed = embed.footer(|f| f.text("Created by 0xdema").icon_url("https://dema.pink/0xdema.png")).to_owned();
    embed = embed.color(0x5500aa).to_owned();

    // Clone the embed contents before using it in send_message
    let cloned_embed = embed.clone();

    // Send the embed
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            // Attach the cloned embed
            *e = cloned_embed;
            e
        })
    }).await {
        println!("Error sending embed: {:?}", why);
    }
}

pub async unsafe fn send_message(ctx: Context, msg: Message, desc: &str) {
    // Create a mutable embed builder
    let mut embed = CreateEmbed::default();
    // Set embed properties
    embed = embed.description(desc).to_owned();
    embed = embed.color(0x5500aa).to_owned();

    // Clone the embed contents before using it in send_message
    let cloned_embed = embed.clone();

    // Send the embed
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            // Attach the cloned embed
            *e = cloned_embed;
            e
        })
    }).await {
        println!("Error sending embed: {:?}", why);
    }
}

pub async fn send_embed_image(ctx: Context, msg: Message, title: &str, desc: &str, path: &str) {
    // Create a mutable embed builder
    let mut embed = CreateEmbed::default();
    // Set embed properties
    embed.title(title);
    embed.description(desc);
    embed.image("attachment://image.jpg");
    embed.color(0x5500aa);

    // Clone the embed contents before using it in send_message
    let cloned_embed = embed.clone();
    let mut file = File::open(path).await.unwrap();

    // Send the embed
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            // Attach the cloned embed
            *e = cloned_embed;
            e
        });
        m.add_file(AttachmentType::File { file: &file, filename: "image.jpg".to_owned() })
    }).await {
        println!("Error sending embed: {:?}", why);
    }

    file.flush().await.unwrap();
}
