use crate::cli::console;
use colored::Colorize;
use serenity::{model::gateway::Ready, prelude::*};

pub async fn ready_tasks(ctx: &Context, ready: Ready) {
    display_connection_status(&ready);
    console::prompt_loop(&ctx, &ready).await;
}

pub fn display_connection_status(ready: &Ready) {
    println!("Client '{}' connected", ready.user.name.to_string().green());
}
