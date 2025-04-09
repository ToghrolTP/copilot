use crate::cli::{
    constants::commands::{HELP, SHOW_GUILDS},
    help::exe_help,
    show_guilds::exe_show_guilds,
};
use serenity::{model::gateway::Ready, prelude::*};
use std::io::{self, Write};

pub async fn prompt_loop(ctx: &Context, ready: &Ready) {
    println!("-h for help");

    handle_commands(ctx, ready).await;
}

pub async fn handle_commands(ctx: &Context, ready: &Ready) {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::with_capacity(64);
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim();
                match trimmed {
                    HELP => exe_help().await,
                    SHOW_GUILDS => exe_show_guilds(&ctx, &ready).await,
                    _ => println!("copilot: Command not found"),
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    println!("Input terminated. Exiting...");
                    break;
                }
            }
        }
    }
}
