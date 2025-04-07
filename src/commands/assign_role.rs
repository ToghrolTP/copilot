use serenity::{model::channel::Message, prelude::*};

pub async fn exe_assign_role(ctx: &Context, msg: &Message) {
    let confirm = serenity::builder::CreateButton::new("confirm_terms_condition")
        .label("Confirm")
        .emoji(serenity::model::channel::ReactionType::Unicode(
            "✅".to_string(),
        ));
    let message = serenity::builder::CreateMessage::new()
        .content("Please confirm all terms and policies of the server!")
        .button(confirm);
    if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
        println!("Error sending message: {:?}", why);
    }
}
