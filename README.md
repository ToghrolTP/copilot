# Copilot Discord Bot (Still under development)

A versatile Discord bot built with Rust using the Serenity framework for Discord API integration. This bot provides various utility commands and role management features for your Discord server.

## Features

- **Basic Commands:**
  - `(0help` - Displays helpful information and server resources
  - `(0ping` - Simple ping/pong command to check if the bot is responsive
  - `(0time` - Shows the current time in Tehran timezone

- **Role Management:**
  - `(0terms` - Creates a message with a confirmation button for server terms
  - `(0setuproles` - Sets up a reaction role system where users can self-assign roles by reacting with emojis
  - Reaction-based role assignment (Red, Green, and Blue team roles)

## Technical Details

This bot is built with:
- Rust programming language
- [Serenity](https://github.com/serenity-rs/serenity) crate for Discord API
- Chrono for time-related functions

## Setup and Configuration

### Prerequisites
- Rust and Cargo installed
- A Discord Bot Token (from [Discord Developer Portal](https://discord.com/developers/applications))

### Environment Variables
Create a `.env` file in the project root with:
```
DISCORD_TOKEN=your_bot_token_here
```

### Installation
1. Clone the repository
2. Install dependencies: `cargo build`
3. Run the bot: `cargo run`

### Configuration
Update constants in `src/constants.rs` with your server's channel and role IDs:
- `ROLE_CHANNEL_ID` - The channel where role selection message will be posted
- `ROLE_MESSAGE_ID` - If you want to update an existing role message instead of creating a new one
- Role IDs for your different team roles

## Deployment

## License

This project is open-source and available under the GPL License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
