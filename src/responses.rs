//Commands
pub const HELP_COMMAND: &str = "!help";
pub const START_COMMAND: &str = "!start";
pub const END_COMMAND: &str = "!end";
pub const RESET_COMMAND: &str = "!reset";
pub const PASS_COMMAND: &str = "!pass";
pub const STATUS_COMMAND: &str = "!status";
pub const STATUS_ALL_COMMAND: &str = "!statusall";
pub const PLUS_COMMAND: char = '+';
pub const MINUS_COMMAND: char = '-';
pub const BOARD_COMMAND: &str = "!board";
pub const COLOR_COMMAND: &str = "!color";
pub const COUNTER_COMMAND: &str = "!counter";
pub const QUIT_COMMAND: &str = "!quit";
//Response Text
pub const HELP_MESSAGE: &str = "**!start** @Player - Starts a new match.
**!end** - Ends the current game.
**!reset** - Starts a new game with the same player.
**!pass **- Passes turn resetting energy to 3.
**!status** - Checks your match information.
**+3** -  Add energy.
**-3 ** - Spend energy.

**Customization **(Lasts until game is **!end** You can **!reset** though.)
**a. !board 15** - Changes the memory gauge. Accepts 1 to 17.
****b. !counter 5**** - Change the energy counter. Accepts 1 to 5.
**c.** Color commands to change your name color:
- **!color Red** - For default colors.
- **!color #ffffff** or **!color #fff** - To use hex color codes
- **!color (100,100,100)** or **!color 100,100,100** - To use RGB color codes.";
//Error Text
pub const PRIVATE_ERROR_MESSAGE: &str = "\r\n\r\nNote: bot will only work in servers.";
pub const NO_PLAYER_ERROR_MESSAGE: &str = "You must @ the person you wish to battle with.";
pub const SELF_ERROR_MESSAGE: &str = "You must may not battle with yourself.";
pub const BOT_ERROR_MESSAGE: &str = "You must may not battle with a bot.";
pub const OTHER_GAME_ERROR_MESSAGE: &str = "You or the other player are already in a game.";
pub const VALID_PLAYER_ERROR_MESSAGE: &str = "Please be sure to ping a valid player.";
pub const NOT_IN_GAME: &str = "You are not currently in any game.";
pub const ALREADY_IN_GAME: &str = "Neither player can be in multiple games.";
pub const WAIT_FOR_TURN: &str = "Please wait until your turn.";
pub const NOT_ENOUGH_ENERGY: &str = "You do not have that much energy to spend";
pub const SELECT_OPTION: &str = "Please select an option from 1-17";
pub const COUNTER_SELECT: &str = "Please select an option from 1-5";
// Other
pub const GAME_START: &str = "Started a game between PLAYER_ONE and PLAYER_TWO.";
pub const CLOSE_GAME: &str = "Your game between PLAYER_ONE & PLAYER_TWO is now closed.";
pub const RESET_GAME: &str = "Your game between PLAYER_ONE and PLAYER_TWO has been reset.";
pub const NOT_YOUR_TURN: &str = "It is PLAYER_TWO's turn.";
pub const TURN_STATUS: &str = "It is the TURN_PLAYER's turn";
pub const FULL_TURN_STATUS: &str =
    "Table: TABLE_ID ~ PLAYER_ONE vs PLAYER_TWO - TURN_PLAYER's turn. ~ TOTAL_ENERGY energy.\r\n";
pub const INVITE_LINK: &str = "https://discord.com/oauth2/authorize?client_id=955931937581723668&scope=bot&permissions=277025639424";
