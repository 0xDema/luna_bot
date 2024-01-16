// Import necessary modules from the standard library and Serenity crate.
use std::process::exit;

use serenity::{model::channel::Message, prelude::*};

// Import local modules.
use crate::core_functions::send_help;
use crate::{core_functions, memory_gauge, responses};

// Static mutable variables to store game data and game count.
pub static mut GAME_DATA: Vec<[i64; 9]> = Vec::new();
// ID1, ID2, Energy, Turn_Player, Gauge1, Gauge2, Counter Image, Color1, Color2
pub static mut GAME_COUNT: u32 = 0;

// Asynchronously handle commands from users.
pub async unsafe fn command(ctx: Context, msg: Message) {
    // Clone the message content and split it into individual words.
    let contents = msg.content.clone();
    let full_command = contents.split_whitespace().collect::<Vec<_>>();
    // Extract the first word (the command) and the remaining arguments.
    let start_of_command = full_command[0];
    let arguments = &contents[start_of_command.len()..];

    // Check if the message is in private chat.
    if msg.is_private() {
        // Send an error message if the command is used in a private chat.
        let out = responses::HELP_MESSAGE.to_string() + responses::PRIVATE_ERROR_MESSAGE;
        core_functions::send_message(ctx, msg, out.as_str()).await;
    } else {
        // Handle different commands based on the first word.
        if contents.clone().remove(0) == responses::PLUS_COMMAND {
            plus(ctx.clone(), msg.clone()).await
        } else if contents.clone().remove(0) == responses::MINUS_COMMAND {
            minus(ctx.clone(), msg.clone()).await
        }

        // Match the command and execute the corresponding function.
        match start_of_command {
            responses::HELP_COMMAND => send_help(ctx, msg).await,
            responses::START_COMMAND => start(ctx, msg, arguments).await,
            responses::END_COMMAND => end(ctx, msg).await,
            responses::RESET_COMMAND => reset(ctx, msg).await,
            responses::PASS_COMMAND => pass(ctx, msg).await,
            responses::STATUS_COMMAND => status(ctx, msg).await,
            responses::STATUS_ALL_COMMAND => status_all(ctx, msg).await,
            responses::BOARD_COMMAND => change_board(ctx, msg, arguments).await,
            responses::COLOR_COMMAND => change_color(ctx, msg, arguments).await,
            responses::COUNTER_COMMAND => change_counter(ctx, msg, arguments).await,
            responses::QUIT_COMMAND => quit().await,
            _ => {}
        }
    }
}

// Function to handle the start command.
async unsafe fn start(ctx: Context, msg: Message, arguments: &str) {
    // Check the game information and respond accordingly.
    let table = core_functions::table_id(msg.clone()).await;
    if table == 18446744073709551615 {
        let game_info = core_functions::in_game(msg.clone(), arguments).await;
        match game_info {
            0 => {
                core_functions::send_message(
                    ctx.clone(),
                    msg.clone(),
                    responses::NO_PLAYER_ERROR_MESSAGE,
                )
                .await
            }
            1 => {
                core_functions::send_message(
                    ctx.clone(),
                    msg.clone(),
                    responses::SELF_ERROR_MESSAGE,
                )
                .await
            }
            2 => {
                core_functions::send_message(ctx.clone(), msg.clone(), responses::BOT_ERROR_MESSAGE)
                    .await
            }
            3 => {
                core_functions::send_message(
                    ctx.clone(),
                    msg.clone(),
                    responses::OTHER_GAME_ERROR_MESSAGE,
                )
                .await
            }
            4 => {
                core_functions::add_game(
                    ctx.clone(),
                    msg.clone(),
                    i64::MAX,
                    0,
                    16,
                    17,
                    4,
                    16711680,
                    255,
                )
                .await
            }
            5 => {
                core_functions::send_message(
                    ctx.clone(),
                    msg.clone(),
                    responses::VALID_PLAYER_ERROR_MESSAGE,
                )
                .await
            }
            _ => {}
        }
    } else {
        core_functions::send_message(ctx.clone(), msg, responses::ALREADY_IN_GAME).await;
    }
}

// Function to handle the end command.
async unsafe fn end(ctx: Context, msg: Message) {
    // Get the table ID and check if the game exists.
    let table = core_functions::table_id(msg.clone()).await;

    println!("{}", table);
    if table != 18446744073709551615 {
        // Retrieve game information
        let game = *GAME_DATA.get(table).unwrap();
        let player_one = core_functions::read_game(game, 0).await;
        let player_two = core_functions::read_game(game, 1).await;
        let player_one = core_functions::to_tag(player_one as u64).await;
        let player_two = core_functions::to_tag(player_two as u64).await;
        let response = responses::CLOSE_GAME
            .replace("PLAYER_ONE", player_one.as_str())
            .replace("PLAYER_TWO", player_two.as_str());
        core_functions::send_message(ctx.clone(), msg, response.as_str()).await;
        // End the game and update game count and bot activity.
        core_functions::end_game(table).await;
        GAME_COUNT -= 1;
        core_functions::change_activity(ctx).await;
    } else {
        core_functions::send_message(ctx.clone(), msg, responses::NOT_IN_GAME).await;
    }
}

// Function to handle the reset command.
async unsafe fn reset(ctx: Context, msg: Message) {
    // Get the table ID and check if the game exists.
    let table = core_functions::table_id(msg.clone()).await;
    if !table == usize::MAX {
        // Reset game values, send reset message, and display the game.
        core_functions::change_value(ctx.clone(), msg.clone(), 2, i64::MAX).await;
        core_functions::change_value(ctx.clone(), msg.clone(), 3, 0).await;
        let game = *GAME_DATA.get(table).unwrap();
        let player_one = core_functions::read_game(game, 0).await;
        let player_two = core_functions::read_game(game, 1).await;
        let player_one = core_functions::to_tag(player_one as u64).await;
        let player_two = core_functions::to_tag(player_two as u64).await;
        let out = responses::RESET_GAME
            .replace("PLAYER_ONE", player_one.as_str())
            .replace("PLAYER_TWO", player_two.as_str());

        //core_functions::send_message(ctx.clone(), msg.clone(), out.as_str()).await;
        core_functions::send_game(ctx, msg, game, table as i32, out.as_str(), "Game Reset").await;
    } else {
        core_functions::send_message(ctx.clone(), msg, responses::NOT_IN_GAME).await;
    }
}

// Function to handle the pass command.
async unsafe fn pass(ctx: Context, msg: Message) {
    // Get the table ID and check if the game exists.
    let table = core_functions::table_id(msg.clone()).await;
    if table != usize::MAX {
        // Retrieve game information and handle passing logic.
        let game = *GAME_DATA.get(table).unwrap();
        let author = core_functions::author(msg.clone());
        let player_one = core_functions::read_game(game, 0).await;
        let player_two = core_functions::read_game(game, 1).await;
        let turn = core_functions::read_game(game, 3).await;
        let player_two_tag = core_functions::to_tag(player_two as u64).await;
        let their_turn = responses::NOT_YOUR_TURN.replace("PLAYER_TWO", player_two_tag.as_str());

        // Check turn and author, update values, and send the game.
        if turn == 0 {
            if author == player_one {
                core_functions::change_value(ctx.clone(), msg.clone(), 2, 3).await;
                core_functions::change_value(ctx.clone(), msg.clone(), 3, 2).await;
            } else if author == player_two {
                core_functions::change_value(ctx.clone(), msg.clone(), 2, -3).await;
                core_functions::change_value(ctx.clone(), msg.clone(), 3, 1).await;
            }
        } else if turn == 1 {
            if author == player_one {
                core_functions::change_value(ctx.clone(), msg.clone(), 2, 3).await;
                core_functions::change_value(ctx.clone(), msg.clone(), 3, 2).await;
            } else if author == player_two {
                core_functions::send_message(ctx.clone(), msg.clone(), their_turn.as_str()).await;
            }
        } else if turn == 2 {
            if author == player_one {
                core_functions::send_message(ctx.clone(), msg.clone(), their_turn.as_str()).await;
            } else if author == player_two {
                core_functions::change_value(ctx.clone(), msg.clone(), 2, -3).await;
                core_functions::change_value(ctx.clone(), msg.clone(), 3, 1).await;
            }
        }
        let game = *GAME_DATA.get(table).unwrap();
        core_functions::send_game(ctx, msg, game, table as i32, "", "Turn Passed").await;
    } else {
        core_functions::send_message(ctx.clone(), msg, responses::NOT_IN_GAME).await;
    }
}

// Function to handle the status command.
async unsafe fn status(ctx: Context, msg: Message) {
    // Get the table ID and check if the game exists.
    let table = core_functions::table_id(msg.clone()).await;
    if table != usize::MAX {
        // Retrieve game information, build the status message, and send it.
        let game = *GAME_DATA.get(table).unwrap();
        let player_one = core_functions::to_tag(*game.first().unwrap() as u64).await;
        let player_two = core_functions::to_tag(*game.get(1).unwrap() as u64).await;
        let turn = core_functions::read_game(game, 3).await;
        let mut output = responses::TURN_STATUS.to_string();

        // Replace placeholders in the status message.
        if turn == 0 {
            output = output.replace("TURN_PLAYER", "the starting player");
        } else if turn == 1 {
            output = output.replace("TURN_PLAYER", &player_one);
        } else if turn == 2 {
            output = output.replace("TURN_PLAYER", &player_two);
        }
        output = output + " (" + &player_one + " v " + &player_two + ")";
        // Send the status message and the game memory gauge.
        //core_functions::send_message(ctx.clone(), msg.clone(), output.as_str()).await;
        core_functions::send_game(ctx, msg, game, table as i32, output.as_str(), "Game Status")
            .await;
    } else {
        core_functions::send_message(ctx.clone(), msg, responses::NOT_IN_GAME).await;
    }
}

// Display information for all ongoing games (admin only)
async unsafe fn status_all(ctx: Context, msg: Message) {
    // Check if the command is invoked by an admin
    if msg.author.tag() == "0xdema#0000" || msg.author.tag() == "duoria#0000" {
        let mut output: String = "".to_string();
        let mut i = 0;

        // Iterate through each ongoing game
        while i < GAME_DATA.len() {
            output += responses::FULL_TURN_STATUS;
            let game = *GAME_DATA.get(i).unwrap();
            let player_one = core_functions::to_tag(*game.first().unwrap() as u64).await;
            let player_two = core_functions::to_tag(*game.get(1).unwrap() as u64).await;
            let player_one = player_one.as_str();
            let player_two = player_two.as_str();
            let mut energy = game.get(2).unwrap().clone().to_string();
            let turn = *game.get(3).unwrap();

            // Populate and format the output string with game details
            output = output.replace("TABLE_ID", i.to_string().as_str());
            output = output.replace("PLAYER_ONE", player_one);
            output = output.replace("PLAYER_TWO", player_two);
            if turn == 0 {
                output = output.replace("TURN_PLAYER", "Starting player");
            } else if turn == 1 {
                output = output.replace("TURN_PLAYER", player_one);
            } else if turn == 2 {
                output = output.replace("TURN_PLAYER", player_two);
            }
            if energy == i64::MAX.to_string() {
                energy = 0.to_string();
            }
            output = output.replace("TOTAL_ENERGY", &energy);
            i += 1;
        }
        if !output.is_empty() {
            core_functions::send_message(ctx, msg, output.as_str()).await;
        }
    }
}

// Decrease energy for a player in a game
async unsafe fn minus(ctx: Context, msg: Message) {
    let arguments = msg.content.replace(['+', '-'], "");
    let table = core_functions::table_id(msg.clone()).await;

    // If game exists
    if table != usize::MAX {
        let game = core_functions::find_game(msg.clone()).await;
        let author = core_functions::author(msg.clone());
        let player_one = core_functions::read_game(game, 0).await;
        let player_two = core_functions::read_game(game, 1).await;
        let mut energy = core_functions::read_game(game, 2).await;
        let turn = core_functions::read_game(game, 3).await;
        let cost: i64 = arguments.parse().unwrap();

        if turn == 0 {
            energy = 0;
            if author == player_one {
                // Handle energy change for player_one
                // Check if energy after change is within valid range
                if energy + cost > 10 {
                    core_functions::send_message(
                        ctx.clone(),
                        msg.clone(),
                        responses::NOT_ENOUGH_ENERGY,
                    )
                    .await;
                } else {
                    energy += cost;
                    core_functions::change_value(ctx.clone(), msg.clone(), 3, 1).await;
                    if energy > 0 {
                        core_functions::change_value(ctx.clone(), msg.clone(), 3, 2).await;
                    }
                }
            } else if author == player_two {
                // Handle energy change for player_two
                // Check if energy after change is within valid range
                if energy - cost < -10 {
                    core_functions::send_message(
                        ctx.clone(),
                        msg.clone(),
                        responses::NOT_ENOUGH_ENERGY,
                    )
                    .await;
                } else {
                    energy -= cost;
                    core_functions::change_value(ctx.clone(), msg.clone(), 3, 2).await;
                    if energy < 0 {
                        core_functions::change_value(ctx.clone(), msg.clone(), 3, 1).await;
                    }
                }
            }
        } else if turn == 1 {
            // Handle energy change for player_one in turn 1
            if author == player_one {
                if energy + cost > 10 {
                    core_functions::send_message(
                        ctx.clone(),
                        msg.clone(),
                        responses::NOT_ENOUGH_ENERGY,
                    )
                    .await;
                } else {
                    energy += cost;
                    if energy > 0 {
                        core_functions::change_value(ctx.clone(), msg.clone(), 3, 2).await;
                    }
                }
            } else if author == player_two {
                // Notify player_two to wait for their turn
                core_functions::send_message(ctx.clone(), msg.clone(), responses::WAIT_FOR_TURN)
                    .await;
            }
        } else if turn == 2 {
            // Handle energy change for player_two in turn 2
            if author == player_one {
                // Notify player_one to wait for their turn
                core_functions::send_message(ctx.clone(), msg.clone(), responses::WAIT_FOR_TURN)
                    .await;
            } else if author == player_two {
                // Check if energy after change is within valid range
                if energy - cost < -10 {
                    core_functions::send_message(
                        ctx.clone(),
                        msg.clone(),
                        responses::NOT_ENOUGH_ENERGY,
                    )
                    .await;
                } else {
                    energy -= cost;
                    if energy < 0 {
                        core_functions::change_value(ctx.clone(), msg.clone(), 3, 1).await;
                    }
                }
            }
        }
        core_functions::change_value(ctx.clone(), msg.clone(), 2, energy).await;
        let game = core_functions::find_game(msg.clone()).await;
        core_functions::send_game(ctx, msg, game, table as i32, "", "").await;
    } else {
        // Notify if the player is not in any game
        core_functions::send_message(ctx, msg, responses::NOT_IN_GAME).await;
    }
}

// Increase energy for a player in a game
async unsafe fn plus(ctx: Context, msg: Message) {
    let arguments = msg.content.replace(['+', '-'], "");
    let table = core_functions::table_id(msg.clone()).await;

    // If game exists
    if table != usize::MAX {
        let game = core_functions::find_game(msg.clone()).await;
        let author = core_functions::author(msg.clone());
        let player_one = core_functions::read_game(game, 0).await;
        let player_two = core_functions::read_game(game, 1).await;
        let mut energy = core_functions::read_game(game, 2).await;
        let turn = core_functions::read_game(game, 3).await;
        let cost: i64 = arguments.parse().unwrap();

        if turn == 0 {
            energy = 0;
            if author == player_one {
                // Handle energy change for player_one
                // Check if energy after change is within valid range
                if energy - cost < -10 {
                    energy = -10;
                } else {
                    energy -= cost;
                    core_functions::change_value(ctx.clone(), msg.clone(), 3, 1).await;
                }
            } else if author == player_two {
                // Handle energy change for player_two
                // Check if energy after change is within valid range
                if energy + cost > 10 {
                    energy = 10;
                } else {
                    energy += cost;
                    core_functions::change_value(ctx.clone(), msg.clone(), 3, 2).await;
                }
            }
        } else if turn == 1 {
            // Handle energy change for player_one in turn 1
            if author == player_one {
                // Check if energy after change is within valid range
                if energy - cost < -10 {
                    energy = -10;
                } else {
                    energy -= cost;
                    core_functions::change_value(ctx.clone(), msg.clone(), 3, 1).await;
                }
            } else if author == player_two {
                // Notify player_two to wait for their turn
                core_functions::send_message(ctx.clone(), msg.clone(), responses::WAIT_FOR_TURN)
                    .await;
            }
        } else if turn == 2 {
            // Handle energy change for player_two in turn 2
            if author == player_one {
                // Notify player_one to wait for their turn
                core_functions::send_message(ctx.clone(), msg.clone(), responses::WAIT_FOR_TURN)
                    .await;
            } else if author == player_two {
                // Check if energy after change is within valid range
                if energy + cost > 10 {
                    energy = 10;
                } else {
                    energy += cost;
                }
            }
        }
        core_functions::change_value(ctx.clone(), msg.clone(), 2, energy).await;
        let game = core_functions::find_game(msg.clone()).await;
        core_functions::send_game(ctx, msg, game, table as i32, "", "").await;
    } else {
        // Notify if the player is not in any game
        core_functions::send_message(ctx, msg, responses::NOT_IN_GAME).await;
    }
}

// Change the memory gauge for a player in a game
async unsafe fn change_board(ctx: Context, msg: Message, arguments: &str) {
    let table = core_functions::table_id(msg.clone()).await;

    if !table == usize::MAX {
        let game = *GAME_DATA.get(table).unwrap();
        // Check if the game is not empty
        if !game.is_empty() {
            let player_one = game.first().unwrap().clone().to_string();
            let player_two = game.get(1).unwrap().clone().to_string();
            let author = core_functions::author(msg.clone()).to_string();
            let arguments = arguments.replace(' ', "");

            if arguments.is_empty() {
                // Notify if no argument is provided
                core_functions::send_message(ctx, msg.clone(), responses::SELECT_OPTION).await;
            } else if arguments.chars().all(char::is_numeric) {
                let arguments = arguments.to_string().parse().unwrap();

                // Numbers that can be accepted
                if !(1..=17).contains(&arguments) {
                    // Notify if the selected gauge is not valid
                    core_functions::send_message(ctx, msg.clone(), responses::SELECT_OPTION).await;
                } else {
                    if author == player_one {
                        // Change gauge for player_one
                        core_functions::change_value(ctx.clone(), msg.clone(), 4, arguments).await;
                    } else if author == player_two {
                        // Change gauge for player_two
                        core_functions::change_value(ctx.clone(), msg.clone(), 5, arguments).await;
                    }
                    let game = core_functions::find_game(msg.clone()).await;
                    core_functions::send_game(
                        ctx,
                        msg,
                        game,
                        table as i32,
                        "",
                        "Memory Gauge Changed",
                    )
                    .await;
                }
            } else {
                // Notify if the argument is not a number
                core_functions::send_message(ctx, msg, responses::SELECT_OPTION).await;
            }
        } else {
            // Notify if the game is empty
            core_functions::send_message(ctx.clone(), msg, responses::NOT_IN_GAME).await;
        }
    } else {
        // Notify if the player is not in any game
        core_functions::send_message(ctx.clone(), msg, responses::NOT_IN_GAME).await;
    }
}

// Function to change the color of a player's name in an ongoing game
async unsafe fn change_color(ctx: Context, msg: Message, arguments: &str) {
    // Get the table ID from the message
    let table = core_functions::table_id(msg.clone()).await;

    // Check if the table ID is valid
    if !table == usize::MAX {
        // Get the game data for the specified table
        let game = GAME_DATA.get(table).unwrap();

        // Check if the game is not empty
        if !game.is_empty() {
            // Get player information
            let player_one = game.first().unwrap().clone().to_string();
            let player_two = game.get(1).unwrap().clone().to_string();
            let author = core_functions::author(msg.clone()).to_string();

            // Remove spaces from arguments
            let arguments = arguments.replace(' ', "");

            // Check if the arguments are empty
            if arguments.is_empty() {
                core_functions::send_message(ctx, msg.clone(), responses::SELECT_OPTION).await;
            } else {
                // Convert the arguments to a bit field using memory_gauge
                let arguments = memory_gauge::to_bit_field(arguments);

                // Check the author and update the corresponding color value
                if author == player_one {
                    core_functions::change_value(ctx.clone(), msg.clone(), 7, arguments).await;
                } else if author == player_two {
                    core_functions::change_value(ctx.clone(), msg.clone(), 8, arguments).await;
                }

                // Get the updated game data and send the updated game state
                let game = core_functions::find_game(msg.clone()).await;
                core_functions::send_game(ctx, msg, game, table as i32, "", "Color Changed").await;
            }
        } else {
            core_functions::send_message(ctx.clone(), msg, responses::NOT_IN_GAME).await;
        }
    } else {
        core_functions::send_message(ctx.clone(), msg, responses::NOT_IN_GAME).await;
    }
}

// Function to change the counter image in an ongoing game
async unsafe fn change_counter(ctx: Context, msg: Message, arguments: &str) {
    // Get the table ID from the message
    let table = core_functions::table_id(msg.clone()).await;

    // Check if the table ID is valid
    if !table == usize::MAX {
        // Get the game data for the specified table
        let game = *GAME_DATA.get(table).unwrap();

        // Check if the game is not empty
        if !game.is_empty() {
            // Remove spaces from arguments
            let arguments = arguments.replace(' ', "");

            // Check if the arguments are empty
            if arguments.is_empty() {
                core_functions::send_message(ctx, msg.clone(), responses::COUNTER_SELECT).await;
            } else if arguments.chars().all(char::is_numeric) {
                // Convert arguments to a numeric value
                let arguments = arguments.to_string().parse().unwrap();

                // Check if the numeric value is within the accepted range
                if !(1..=5).contains(&arguments) {
                    core_functions::send_message(ctx, msg.clone(), responses::COUNTER_SELECT).await;
                } else {
                    // Update the counter image
                    core_functions::change_value(ctx.clone(), msg.clone(), 6, arguments).await;

                    // Get the updated game data and send the updated game state
                    let game = core_functions::find_game(msg.clone()).await;
                    core_functions::send_game(ctx, msg, game, table as i32, "", "Counter Changed")
                        .await;
                }
            } else {
                core_functions::send_message(ctx, msg, responses::COUNTER_SELECT).await;
            }
        } else {
            core_functions::send_message(ctx.clone(), msg, responses::NOT_IN_GAME).await;
        }
    } else {
        core_functions::send_message(ctx.clone(), msg, responses::NOT_IN_GAME).await;
    }
}

// Function to quit the program
async fn quit() {
    exit(0);
}
