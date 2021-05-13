use diesel::prelude::*;
use rand::prelude::*;
use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
};
use tracing::info;

use crate::{
    DatabaseConnection,
    schema::amimir_urls::dsl::*,
};

#[command]
pub async fn amimir(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let rand_url: String = {
        let data = ctx.data.read().await;
        let db = data.get::<DatabaseConnection>().unwrap()
            .lock().await;

        get_random_amimir(&db)?
    };
    msg.channel_id.say(&ctx.http, rand_url).await?;
    Ok(())
}

fn get_random_amimir(db: &SqliteConnection) -> QueryResult<String> {
    let count: i64 = amimir_urls.count().get_result(db)?;
    info!("amimir count: {}", count);

    let mut rng = thread_rng();
    let offset: i64 = rng.gen_range(0 .. count);
    
    amimir_urls.select(url).order(id).offset(offset).first(db)
}
