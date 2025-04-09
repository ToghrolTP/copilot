mod bot;
mod cli;
mod commands;
mod constants;
mod handlers;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    if let Err(why) = bot::start(token).await {
        println!("Client error{:?}", why);
    }
}
