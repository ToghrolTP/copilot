use crate::commands::reaction_roles::ReactionRoles;
use crate::constants::ROLE_MESSAGE_ID;
use serenity::model::channel::ReactionType;
use serenity::{model::channel::Reaction, prelude::*};

pub async fn handle_reaction_add(ctx: Context, reaction: Reaction) {
    handle_reaction(&ctx, &reaction, true).await;
}

pub async fn handle_reaction_remove(ctx: Context, reaction: Reaction) {
    handle_reaction(&ctx, &reaction, false).await;
}

async fn handle_reaction(ctx: &Context, reaction: &Reaction, is_add: bool) {
    if reaction.message_id != ROLE_MESSAGE_ID {
        return;
    }

    let user_id = match reaction.user_id {
        Some(id) => id,
        None => return,
    };
    if let Ok(bot_user) = ctx.http.get_current_user().await {
        if user_id == bot_user.id {
            return;
        }
    }

    // Get the role mappings
    let data = ctx.data.read().await;
    let mappings = match data.get::<ReactionRoles>() {
        Some(map) => map,
        None => {
            println!("No role mappings found");
            return;
        }
    };

    // Convert reaction emoji to the same format as stored in the HashMap
    let emoji_string = match &reaction.emoji {
        ReactionType::Unicode(s) => s.clone(),
        _ => {
            println!("Custom emoji not supported for role assignment");
            return;
        }
    };

    println!("Looking for emoji: {}", emoji_string); // Debug print

    // Find matching role for the emoji
    let role_id = match mappings.get(&emoji_string) {
        Some(role) => role,
        None => {
            println!("No role mapping for emoji: {}", emoji_string);
            return;
        }
    };

    // Get guild and member
    let guild_id = match reaction.guild_id {
        Some(id) => id,
        None => return,
    };

    let member = match guild_id.member(&ctx.http, user_id).await {
        Ok(m) => m,
        Err(e) => {
            println!("Error getting member: {:?}", e);
            return;
        }
    };

    // Add or remove role based on is_add flag
    if is_add {
        if let Err(e) = member.add_role(&ctx.http, role_id).await {
            println!("Error adding role: {:?}", e);
        } else {
            println!("Added role {:?} to user {}", role_id, user_id);
        }
    } else if let Err(e) = member.remove_role(&ctx.http, role_id).await {
        println!("Error removing role: {:?}", e);
    } else {
        println!("Removed role {:?} from user {}", role_id, user_id);
    }
}
