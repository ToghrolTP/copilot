use colored::Colorize;
use prettytable::{row, Cell, Table};
use serenity::{model::gateway::Ready, prelude::*};

pub async fn ready_tasks(ctx: &Context, ready: Ready) {
    println!("Client '{}' connected", ready.user.name.to_string().green());

    show_guilds(ctx, ready).await;
}

pub async fn show_guilds(ctx: &Context, ready: Ready) {
    let mut table = Table::new();

    // titles
    table.set_titles(row![Cell::new("ID"), Cell::new("Name"), Cell::new("Owner"),]);

    // rows
    for guild in ready.guilds {
        if guild.unavailable {
            match guild.id.to_partial_guild(&ctx.http).await {
                Ok(partial_guild) => {
                    let id_string = partial_guild.id.to_string().blue().to_string();
                    let name_string = partial_guild.name.clone().blue().to_string();
                    let owner_id_string = partial_guild.owner_id.to_string().blue().to_string();

                    table.add_row(row![
                        Cell::new(&id_string),
                        Cell::new(&name_string),
                        Cell::new(&owner_id_string),
                    ]);
                }
                Err(e) => {
                    eprintln!("Server {} Error fetching partial guild: {:?}", guild.id, e);
                }
            }
        }
    }

    table.printstd();
}
