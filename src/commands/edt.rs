use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
};

#[command]
pub async fn edt(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.reply(ctx, "bientôt™").await?;
    Ok(())
}
