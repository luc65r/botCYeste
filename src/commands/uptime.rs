use std::time::Duration;

use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
};

use crate::{
    Uptime,
    utils::format_duration,
};

#[command]
pub async fn uptime(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let time: Duration = {
        let data = ctx.data.read().await;
        let start_time = data.get::<Uptime>()
            .ok_or("cannot get global data uptime")?;
        start_time.elapsed()?
    };

    msg.reply(ctx, format!("Le bot est actif depuis {}.", format_duration(time)))
        .await?;

    Ok(())
}
