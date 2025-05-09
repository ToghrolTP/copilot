mod message;
mod reaction;
mod ready;

use serenity::{
    async_trait,
    model::{
        channel::{Message, Reaction},
        gateway::Ready,
    },
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(e) = message::handle_message(ctx, msg).await {
            eprintln!("Error handling message: {:?}", e);
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready_tasks(&ctx, ready).await;
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        reaction::handle_reaction_add(ctx, add_reaction).await;
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        reaction::handle_reaction_remove(ctx, reaction).await;
    }
}
