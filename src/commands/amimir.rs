use std::time::Duration;

use diesel::{
    prelude::*,
    dsl::{select, exists, insert_into, delete, max},
};
use rand::prelude::*;
use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
};
use tracing::{info, warn};

use crate::{
    DatabaseConnection,
    schema::amimir_urls::dsl::*,
};

macro_rules! with_db {
	($ctx:ident, $closure:expr) => {
		{
            let data = $ctx.data.read().await;
            let db = data.get::<DatabaseConnection>().unwrap()
                .lock().await;
            $closure(&db)
        }
	};
}

#[command]
pub async fn amimir(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: errors
    match args.single::<String>().as_deref() {
        Ok("add") => {
            if let Ok(new_url) = args.single::<String>() {
                if check_url(&new_url).await {
                    let inserted = with_db!(ctx, |db| insert_amimir(db, &new_url))?;
                    if inserted {
                        msg.react(&ctx.http, ReactionType::Unicode(String::from("👍")))
                            .await?;
                    } else {
                        msg.reply(&ctx.http, format!("<{}> est déjà dans la liste des liens", new_url))
                            .await?;
                    }
                } else {
                    warn!("{} isn't a valid link", new_url);
                    msg.reply(&ctx.http, format!(
                        "<{}> n'est pas un lien valide, ou le site n'est pas atteignable",
                        new_url,
                    )).await?;
                }
            } else {
                warn!("amimir add without url");
                msg.reply(&ctx.http, "Il manque le lien").await?;
            }
        },

        Ok("del") => {
            if let Ok(old_url) = args.single::<String>() {
                let deleted = with_db!(ctx, |db| delete_amimir(db, &old_url))?;
                if deleted {
                    msg.react(&ctx.http, ReactionType::Unicode(String::from("👍")))
                        .await?;
                } else {
                    msg.reply(&ctx.http, format!("<{}> n'était pas dans la liste", old_url))
                        .await?;
                }
            } else {
                let deleted_url = with_db!(ctx, delete_last_amimir)?;
                msg.reply(&ctx.http, if let Some(u) = deleted_url {
                    format!("<{}> supprimé", u)
                } else {
                    String::from("Auncun lien n'a été supprimé")
                }).await?;
            }
        },

        Ok(s) => {
            warn!("unknown amimir command: {}", s);
            msg.reply(&ctx.http, format!("Commande `{}` inconnue", s))
                .await?;
        },

        Err(_) => {
            let rand_url: String = with_db!(ctx, get_random_amimir)?;
            msg.channel_id.say(&ctx.http, rand_url).await?;
        },
    }

    Ok(())
}

fn get_random_amimir(db: &SqliteConnection) -> QueryResult<String> {
    let count: i64 = amimir_urls.count().get_result(db)?;
    info!("amimir count: {}", count);

    let mut rng = thread_rng();
    let offset: i64 = rng.gen_range(0 .. count);

    amimir_urls.select(url).order(id).offset(offset).first(db)
}

fn insert_amimir(db: &SqliteConnection, new_url: &str) -> QueryResult<bool> {
    let exists = select(exists(amimir_urls.filter(url.eq(&new_url))))
        .get_result(db)?;
    if exists {
        warn!("amimir url {} is already known", new_url);
        Ok(false)
    } else {
        info!("inserting {} into amimir urls", new_url);
        insert_into(amimir_urls)
            .values(url.eq(new_url))
            .execute(db)?;
        Ok(true)
    }
}

fn delete_amimir(db: &SqliteConnection, old_url: &str) -> QueryResult<bool> {
    info!("deleting {} from amimir urls", old_url);
    let nb_rows = delete(amimir_urls.filter(url.eq(old_url)))
        .execute(db)?;

    if nb_rows == 0 {
        warn!("{} wasn't present in amimir urls", old_url);
        Ok(false)
    } else {
        info!("deleted {} from amimir urls", old_url);
        Ok(true)
    }
}

fn delete_last_amimir(db: &SqliteConnection) -> QueryResult<Option<String>> {
    if let Some(max_id) = amimir_urls.select(max(id)).first::<Option<i32>>(db)? {
        let deleted_url: String = amimir_urls.select(url)
            .filter(id.eq(max_id)).first(db)?;
        info!("deleting {} from amimir urls", deleted_url);
        delete(amimir_urls.filter(id.eq(max_id)))
            .execute(db)?;
        Ok(Some(deleted_url))
    } else {
        warn!("cannot get max id");
        Ok(None)
    }
}

async fn check_url(u: &str) -> bool {
    info!("checking if {} is a valid link", u);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build().unwrap();
    client.get(u).send().await.map_or(false, |r| r.status().is_success())
}
