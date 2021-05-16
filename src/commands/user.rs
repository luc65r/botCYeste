use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
};
use tracing::warn;

#[command]
pub async fn user(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let user: User = {
        if args.is_empty() {
            msg.author.clone()
        } else if let Some(u) = msg.mentions.get(0) {
            u.clone()
        } else if let Ok(uid) = args.parse::<UserId>() {
            if let Ok(u) = uid.to_user(ctx).await {
                u
            } else {
                warn!("{} doesn't correspond to any user", uid);
                msg.reply(ctx, format!("{} n'est l'ID d'aucun utilisateur", uid))
                    .await?;
                return Ok(());
            }
        } else {
            let arg = args.message();
            warn!("couldn't get an user from {}", arg);
            msg.reply(ctx, format!("Impossible de récupérer l'utilisateur {}", arg))
                .await?;
            return Ok(());
        }
    };

    let nickname = user.nick_in(
        ctx, msg.guild_id.ok_or("not in a guild")?
    ).await;

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title(user.tag());
            e.field("ID", user.id.to_string(), false);
            if let Some(n) = nickname {
                e.field("Nickname", n, false);
            }
            if let Some(avatar) = user.avatar_url() {
                e.image(avatar);
            }

            e
        });

        m
    }).await?;

    Ok(())
}
