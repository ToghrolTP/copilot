use chrono::prelude::*;
use chrono_tz::Tz;
use serenity::{model::channel::Message, prelude::*};

pub async fn exe_time(ctx: &Context, msg: &Message) {
    let tehran: Tz = chrono_tz::Asia::Tehran;
    let current_time = Local::now().with_timezone(&tehran);

    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            format!(
                "Current time of Tehran: **{}**",
                current_time.format("%H:%M:%S")
            ),
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}
