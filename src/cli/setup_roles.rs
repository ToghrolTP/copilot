use std::collections::HashMap;
use std::io::{self, Write};

use serenity::all::{CreateEmbed, CreateMessage, EditMessage, GuildId, MessageId};
use serenity::{
    all::{ChannelId, RoleId},
    prelude::*,
};

use crate::commands::reaction_roles::ReactionRoles;

pub async fn exe_setup_roles(ctx: &Context) {
    let channel_id = prompt_for_channel_id();
    if channel_id.is_none() {
        return;
    }

    let message_id = prompt_for_message_id();

    let title = prompt_for_input("Enter title for embed: ");
    let description = prompt_for_input("Enter description (or leave empty): ");

    let role_mapping = collect_role_mapping(ctx).await;

    execute_reaction_roles(
        ctx,
        channel_id,
        message_id,
        title,
        description,
        role_mapping,
    );
}

fn prompt_for_channel_id() -> Option<ChannelId> {
    println!("Enter the channel ID for the role message: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("Failed to read input");
        return None;
    }

    match input.trim().parse::<u64>() {
        Ok(id) => Some(ChannelId::new(id)),
        Err(_) => {
            println!("Invalid channel ID format. Please enter a valid numeric ID.");
            None
        }
    }
}

fn prompt_for_message_id() -> Option<MessageId> {
    println!("Do you want to update an existing message? (y/n)");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut response = String::new();
    if io::stdin().read_line(&mut response).is_err() {
        println!("Failed to read input");
        return None;
    }

    match response.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            print!("Enter the message ID to update: ");
            io::stdout().flush().unwrap();

            let mut id_input = String::new();
            if io::stdin().read_line(&mut id_input).is_err() {
                println!("Failed to read input");
                return None;
            }

            match id_input.trim().parse::<u64>() {
                Ok(id) => return Some(MessageId::new(id)),
                Err(_) => {
                    println!("Invalid message ID format. Please enter a valid numeric ID.");
                    None
                }
            }
        }
        "n" | "no" => return None,
        _ => {
            println!("Please answer with 'y' or 'n'");
            None
        }
    }
}

fn prompt_for_guild_id() -> Option<GuildId> {
    print!("Enter the guild ID for role validation: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("Failed to read input");
        return None;
    }

    match input.trim().parse::<u64>() {
        Ok(id) => Some(GuildId::new(id)),
        Err(_) => {
            println!("Invalid guild ID format. Please enter a valid numeric ID.");
            None
        }
    }
}

fn prompt_for_input(prompt: &str) -> Result<String, io::Error> {
    loop {
        print!("{}", prompt);
        io::stdout().flush()?;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed_input = input.trim();
                if !trimmed_input.is_empty() {
                    return Ok(trimmed_input.to_string());
                }
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {
                continue;
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
}

/// Collects emoji to role mappings interactively from the user
///
/// This function prompts the user to enter role IDs and corresponding emojis,
/// validates the roles exist in the guild, and creates a mapping for reaction roles.
///
/// # Arguments
///
/// * `ctx` - The Discord client context
///
/// # Returns
///
/// A HashMap mapping emoji strings to Discord role IDs
async fn collect_role_mapping(ctx: &Context) -> HashMap<String, RoleId> {
    let mut mapping = HashMap::new();

    // First, we need to get the guild ID to validate roles
    let guild_id = match prompt_for_guild_id() {
        Some(id) => id,
        None => {
            println!("Invalid guild ID. Cannot continue with role setup.");
            return mapping;
        }
    };

    // Fetch all roles in the guild to validate role IDs
    let guild_roles = match guild_id.roles(&ctx.http).await {
        Ok(roles) => roles,
        Err(e) => {
            println!("Error fetching guild roles: {:?}", e);
            return mapping;
        }
    };

    println!("Enter role information. Type 'done' for the role ID when finished.");

    loop {
        // 1. Prompt for role ID
        let role_input = match prompt_for_input("Enter role ID (or 'done' to finish): ") {
            Ok(input) => input,
            Err(e) => {
                println!("Error reading input: {:?}", e);
                continue;
            }
        };

        // If "done", break the loop
        if role_input.to_lowercase() == "done" {
            break;
        }

        // Parse the role ID
        let role_id = match role_input.parse::<u64>() {
            Ok(id) => RoleId::new(id),
            Err(_) => {
                println!("Invalid role ID format. Please enter a valid numeric ID.");
                continue;
            }
        };

        // 3. Validate role exists in guild
        if !guild_roles.contains_key(&role_id) {
            println!(
                "Role ID {} does not exist in the guild. Please enter a valid role ID.",
                role_id
            );
            continue;
        }

        // 2. Prompt for emoji
        let emoji = match prompt_for_input("Enter emoji for this role: ") {
            Ok(input) => input,
            Err(e) => {
                println!("Error reading input: {:?}", e);
                continue;
            }
        };

        // Validate emoji (simple check to ensure it's not empty)
        if emoji.is_empty() {
            println!("Emoji cannot be empty. Please enter a valid emoji.");
            continue;
        }

        // 4. Add to mappings
        println!(
            "Adding role mapping: {} -> {}",
            emoji,
            guild_roles.get(&role_id).unwrap().name
        );
        mapping.insert(emoji, role_id);

        // Optional: Display current mappings
        println!("Current mappings:");
        for (emoji, role_id) in &mapping {
            if let Some(role) = guild_roles.get(role_id) {
                println!("  {} -> {} ({})", emoji, role.name, role_id);
            }
        }
    }

    if mapping.is_empty() {
        println!("Warning: No role mappings were created.");
    } else {
        println!("Successfully created {} role mappings.", mapping.len());
    }

    mapping
}

async fn execute_reaction_roles(
    ctx: &Context,
    channel_id: Option<ChannelId>,
    message_id: Option<MessageId>,
    title: Result<String, io::Error>,
    description: Result<String, io::Error>,
    role_mapping: HashMap<String, RoleId>,
) {
    // Unwrap results and handle errors
    let title = match title {
        Ok(title) => title,
        Err(e) => {
            eprintln!("Error reading title input: {}", e);
            return;
        }
    };

    let description = match description {
        Ok(desc) => desc,
        Err(e) => {
            eprintln!("Error reading description input: {}", e);
            return;
        }
    };

    // Ensure we have a channel ID
    let channel_id = match channel_id {
        Some(id) => id,
        None => {
            eprintln!("Missing channel ID, cannot setup reaction roles");
            return;
        }
    };

    // Store role mapping in the context data
    {
        let mut data = ctx.data.write().await;
        data.insert::<ReactionRoles>(role_mapping.clone());
    }

    // Create the embed with proper string values
    let embed = CreateEmbed::default()
        .title(title)
        .description(description)
        .color(0x00ff00);

    // Now determine if we're updating an existing message or creating a new one
    if let Some(msg_id) = message_id {
        // Updating existing message
        match channel_id.message(&ctx.http, msg_id).await {
            Ok(mut existing_msg) => {
                println!("Found existing message. Updating it with new content.");

                // Edit the existing message with our new content
                let edit_message = EditMessage::default()
                    .content("**Choose your roles by reacting to this message:**")
                    .embed(embed);

                if let Err(why) = existing_msg.edit(&ctx.http, edit_message).await {
                    eprintln!("Error updating existing message: {:?}", why);
                    return;
                }

                // Clear existing reactions if any
                if let Err(why) = existing_msg.delete_reactions(&ctx.http).await {
                    eprintln!("Error clearing existing reactions: {:?}", why);
                    // Continue anyway, as we might still be able to add new reactions
                }

                // Add reactions to the message
                for emoji in role_mapping.keys() {
                    // Try to get the first character of the emoji string
                    if let Some(emoji_char) = emoji.chars().next() {
                        if let Err(why) = existing_msg.react(&ctx.http, emoji_char).await {
                            eprintln!("Error adding reaction {}: {:?}", emoji, why);
                        }
                    } else {
                        eprintln!("Invalid emoji string: {}", emoji);
                    }
                }

                println!("Successfully updated reaction role message!");
            }
            Err(e) => {
                eprintln!("Error fetching message to update: {:?}", e);
                eprintln!("Will attempt to create a new message instead.");

                // Fall through to create a new message
                create_new_message(ctx, channel_id, embed, &role_mapping).await;
            }
        }
    } else {
        // Creating a new message
        create_new_message(ctx, channel_id, embed, &role_mapping).await;
    }
}

async fn create_new_message(
    ctx: &Context,
    channel_id: ChannelId,
    embed: CreateEmbed,
    role_mapping: &HashMap<String, RoleId>,
) {
    println!("Creating a new reaction role message.");

    let message = CreateMessage::default()
        .content("**Choose your roles by reacting to this message:**")
        .embed(embed);

    match channel_id.send_message(&ctx.http, message).await {
        Ok(new_msg) => {
            println!("Successfully created new message with ID: {}", new_msg.id);

            // Add reactions to the new message
            for emoji in role_mapping.keys() {
                if let Some(emoji_char) = emoji.chars().next() {
                    if let Err(why) = new_msg.react(&ctx.http, emoji_char).await {
                        eprintln!("Error adding reaction {}: {:?}", emoji, why);
                    }
                }
            }

            println!("Message ID: {}. Save this for future updates.", new_msg.id);
            println!("Reaction role setup complete!");
        }
        Err(why) => {
            eprintln!("Error creating reaction role message: {:?}", why);
        }
    }
}
