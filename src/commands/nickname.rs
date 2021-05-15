use std::time::Duration;

use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
};
use tokio::time::sleep;
use tracing::{info, warn};

#[command]
pub async fn nickname(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.reply(
        ctx,
        "https://cdn.discordapp.com/attachments/750673510656901223/842843913789112340/nivkname.jpg",
    ).await?;

    let answer = msg.channel_id.await_reply(ctx).await
        .ok_or("didn't get a reply")?;
    let guild = msg.guild_id
        .ok_or("not in a guild")?;

    let new_nickname: String = answer.content
        .trim()
        .chars().take(32) // Discord's limit
        .collect();

    info!(
        "{} changed {} nickname to {}",
        answer.author, msg.author, new_nickname,
    );
    guild.edit_member(ctx, msg.author.id, |m| m.nickname(&new_nickname))
        .await?;

    sleep(Duration::from_secs(3 * 60 * 60)).await;
    if msg.author_nick(ctx).await == Some(new_nickname) {
        guild.edit_member(ctx, msg.author.id, |m| m.nickname(""))
            .await?;
    } else {
        warn!("{} nickname changed, not changing back", msg.author);
    }

    Ok(())
}
