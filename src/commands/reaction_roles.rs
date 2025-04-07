use std::collections::HashMap;

use serenity::all::EditMessage;
use serenity::prelude::*;
use serenity::{
    builder::{CreateEmbed, CreateMessage},
    model::{
        channel::Message,
        id::{ChannelId, RoleId},
    },
    prelude::TypeMapKey,
};

use crate::constants::{
    BLUE_ROLE_ID, GREEN_ROLE_ID, RED_ROLE_ID, ROLE_CHANNEL_ID, ROLE_MESSAGE_ID,
};

pub struct ReactionRoles;

impl TypeMapKey for ReactionRoles {
    type Value = HashMap<String, RoleId>;
}

pub async fn exe_setup_reaction_roles(ctx: &Context, msg: &Message) {
    let channel_id = ChannelId::new(ROLE_CHANNEL_ID);

    // Create the role selection message
    let role_mapping = {
        let mut map = HashMap::new();
        map.insert("🔴".to_string(), RoleId::new(RED_ROLE_ID)); // Red role
        map.insert("🟢".to_string(), RoleId::new(GREEN_ROLE_ID)); // Green role
        map.insert("🔵".to_string(), RoleId::new(BLUE_ROLE_ID)); // Blue role
        map
    };

    // Store role mapping in the context data
    {
        let mut data = ctx.data.write().await;
        data.insert::<ReactionRoles>(role_mapping.clone());
    }

    // Send the role selection message
    let embed = CreateEmbed::default()
        .title("Available Roles")
        .description("🔴 - Red Team\n🟢 - Green Team\n🔵 - Blue Team")
        .color(0x00ff00);

    let _message = CreateMessage::default()
        .content("**Chose your roles by reacting to this message:**")
        .embed(embed.clone());

    // If referring to existing message
    // Check if we have an existing message ID
    if ROLE_MESSAGE_ID != 0 {
        match channel_id.message(&ctx.http, ROLE_MESSAGE_ID).await {
            Ok(mut existing_msg) => {
                println!("Found existing role message. Updating it instead of creating a new one.");

                // Edit the existing message with our new content
                let edit_message = EditMessage::default().embed(embed.clone());
                if let Err(why) = existing_msg.edit(&ctx.http, edit_message).await {
                    println!("Error updating existing message: {:?}", why);
                    // Continue to create a new message if update fails
                } else {
                    // Add reactions to the existing message
                    for emoji in role_mapping.keys() {
                        if let Some(emoji_char) = emoji.chars().next() {
                            if let Err(why) = existing_msg.react(&ctx.http, emoji_char).await {
                                println!("Error adding reaction {}: {:?}", emoji, why);
                            }
                        }
                    }

                    // Message was successfully updated, inform user and return
                    if let Err(why) = msg
                        .channel_id
                        .say(&ctx.http, "Role selection message has been updated!")
                        .await
                    {
                        println!("Error sending confirmation: {:?}", why);
                    }
                }
            }
            Err(_) => {
                println!("Could not find existing role message. Creating a new one.");
            }
        };
    }
}
