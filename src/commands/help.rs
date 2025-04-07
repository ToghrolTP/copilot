use crate::constants::HELP_MESSAGE;
use serenity::{model::channel::Message, prelude::*};

pub async fn exe_help(ctx: &Context, msg: &Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
        println!("Error sending help message: {:?}", why);
    }
}
