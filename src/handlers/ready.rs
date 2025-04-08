use colored::Colorize;
use serenity::{model::gateway::Ready, prelude::*};

pub async fn ready_tasks(ctx: &Context, ready: Ready) {
    println!("Client '{}' connected", ready.user.name.to_string().green());

    for guild in ready.guilds {
        if guild.unavailable {
            match guild.id.to_partial_guild(&ctx.http).await {
                Ok(partial_guild) => {
                    println!(
                        "Server: {} Name: {} {}",
                        guild.id,
                        partial_guild.name,
                        "(unavailable)".yellow()
                    );
                }
                Err(e) => {
                    eprintln!("Server {} Error fetching partial guild: {:?}", guild.id, e);
                }
            }
        }
    }
}
