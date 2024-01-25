# Lunabot - Digimon TCG Memory Gauge Discord Bot

Lunabot is a Discord bot designed to enhance the Digimon Trading Card Game (TCG) experience by providing a customizable digital interactable memory gauge to assist with online and offline play.

[Click here](https://discord.com/oauth2/authorize?client_id=955931937581723668&scope=bot&permissions=277025639424) to add to your server.

## Features

- **Memory Gauge Tracking:** Lunabot helps players keep track of their memory gauge during Digimon TCG matches.

- **Command Prefix:** The default command prefix is `!`, but it can be customized.

## Building from Source

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
- An existing discord bot created in https://discord.com/developers/applications to be used.

### Installation

1. Put your font of choice in [Templates](src/Templates) named font.ttf
   
2. Put your counters in [Counters](src/Counters) named 1.png... Then add them in the function get_counter_bytes() in [memory_gauge.rs](src/memory_gauge.rs). Then change number of counters in change_counter() in [commands.rs](src/commands.rs) (Or stick with default 5)
   
3. Put your gauges in [Gauges](src/Gauges) named 1.jpg... Then add them in the function get_gauge_bytes() in [memory_gauge.rs](src/memory_gauge.rs). Then change number of counters in change_gauge() in [commands.rs](src/commands.rs) (Or stick with default 15)

4. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/lunabot.git
5. Navigate to the project directory:
   ```bash
   git cargo build --release
6. Add the bot secret token. (Replace TOKEN with your token)
   ```bash
   export LUNABOT_TOKEN=TOKEN
7. You will want to put a folder beside the executable called Table for temporary storage of gauge images.

## Commands

**!start @Player** - Starts a new match.

**!end** - Ends the current game.

**!reset** - Starts a new game with the same player.

**!pass** - Passes turn resetting energy to 3.

**!status** - Checks your match information.

**+3** - Add energy.

**-3** - Spend energy.


#### Customization (Lasts until game is !end You can !reset though.)

**a. !board 15** - Changes the memory gauge. Accepts 1 to 17.

**b. !counter 5** - Change the energy counter. Accepts 1 to 5.

**c.** Color commands to change your name color:

**!color Red** - For default colors.

**!color #ffffff** or **!color #fff** - To use hex color codes

**!color (100,100,100)** or **!color 100,100,100** - To use RGB color codes.

## Contributing

If you'd like to contribute to Lunabot, feel free to open an issue or submit a pull request. All contributions are welcome!

Currently we could really could be assisted by any artists interested in making more custom memory gauges, counters, and a profile picture for the bot.

The counters are (80px,80px), the memory gauges are (500px,350px). 2 examples of psd's with placement of memory gauge and template are in the main directory.

On the code side optimization, code cleanup, and bug fixes are the current priority however there are some features being considered for future updates.

## License

This project is licensed under the [MIT License](LICENSE).
