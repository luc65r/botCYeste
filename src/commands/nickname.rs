use std::{
    error::Error,
    time::Duration,
    borrow::Cow,
    io::Cursor,
};

use image::{
    ImageFormat, ImageOutputFormat,
    Rgba,
    io::Reader,
};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
    http::AttachmentType,
};
use tokio::time::sleep;
use tracing::{info, warn};

#[command]
pub async fn nickname(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let user: &User = msg.mentions.get(0)
        .unwrap_or(&msg.author);

    let image = Cow::from(gen_image(&user.tag())?);
    msg.channel_id.send_message(ctx, |m| {
        m.add_file(AttachmentType::Bytes {
            data: image,
            filename: String::from("nickname.png"),
        })
    }).await?;

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
        answer.author, user, new_nickname,
    );
    guild.edit_member(ctx, user.id, |m| m.nickname(&new_nickname))
        .await?;

    sleep(Duration::from_secs(3 * 60 * 60)).await;
    if user.nick_in(ctx, guild).await == Some(new_nickname) {
        guild.edit_member(ctx, user.id, |m| m.nickname(""))
            .await?;
    } else {
        warn!("{} nickname changed, not changing back", user);
    }

    Ok(())
}

fn gen_image(tag: &str) -> Result<Vec<u8>, Box<dyn Error + Sync + Send>> {
    let font_data: &[u8] = include_bytes!("../../res/DejaVuSans.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();

    let template_data: &[u8] = include_bytes!("../../res/nickname.png");
    let mut image = Reader::with_format(Cursor::new(template_data), ImageFormat::Png)
        .decode()?;

    draw_text_mut(
        &mut image,
        Rgba([0, 0, 0, 255]),
        10, 10,
        Scale::uniform(26.0),
        &font,
        "Le message ci-dessous sera le pseudo de",
    );

    draw_text_mut(
        &mut image,
        Rgba([0, 0, 0, 255]),
        40, 50,
        Scale::uniform(32.0),
        &font,
        tag,
    );

    draw_text_mut(
        &mut image,
        Rgba([0, 0, 0, 255]),
        200, 100,
        Scale::uniform(20.0),
        &font,
        "pendant 3h",
    );

    let mut png: Vec<u8> = Vec::new();
    image.write_to(&mut png, ImageOutputFormat::Png)?;

    Ok(png)
}
