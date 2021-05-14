use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
};
use tracing::{info, warn, error};

#[command]
pub async fn nickname(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    info!("{} asked to change nickname", msg.author);
    msg.reply(
        &ctx,
        "https://cdn.discordapp.com/attachments/750675988534394951/842066518576857155/alllah.png",
    ).await?;

    if let Some(answer) = msg.channel_id.await_reply(&ctx).await {
        if let Some(guild) = msg.guild_id {
            info!(
                "{} changed {} nickname to {}",
                answer.author, msg.author, answer.content,
            );
            if let Err(err) = guild.edit_member(
                &ctx, msg.author.id,
                |m| m.nickname(&answer.content)
            ).await {
                error!("couldn't change {} nickname: {}", msg.author, err);
            };
        } else {
            warn!("{} asked to change nickname not in a guild", msg.author);
        }
    } else {
        error!("{} didn't recieve an answer to change his nickname", msg.author);
    }

    Ok(())
}
