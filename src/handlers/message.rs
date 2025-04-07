use crate::commands::{exe_assign_role, exe_help, exe_ping, exe_setup_reaction_roles, exe_time};
use serenity::{model::channel::Message, prelude::*};

use crate::constants::commands;

pub async fn handle_message(
    ctx: Context,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg.content.as_str() {
        commands::HELP => exe_help(&ctx, &msg).await,
        commands::PING => exe_ping(&ctx, &msg).await,
        commands::TIME => exe_time(&ctx, &msg).await,
        commands::CONFIRM_TERMS => exe_assign_role(&ctx, &msg).await,
        commands::SETUP_REACTION_ROLES => exe_setup_reaction_roles(&ctx, &msg).await,
        _ => {}
    }

    Ok(())
}
