#[macro_use]
extern crate diesel;

mod commands;
mod schema;
mod models;

use std::{
    env,
};

use diesel::{
    prelude::*,
    sqlite::SqliteConnection,
};
use dotenv::dotenv;
use serenity::{
    async_trait,
    prelude::*,
    model::prelude::*,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
};
use tracing::info;
use tracing_subscriber::{FmtSubscriber, EnvFilter};

use commands::{
    amimir::*,
    nickname::*,
};

struct DatabaseConnection;

impl TypeMapKey for DatabaseConnection {
    type Value = Mutex<SqliteConnection>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(amimir, nickname)]
struct General;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let db_url = env::var("DATABASE_URL").unwrap();
    let conn = SqliteConnection::establish(&db_url).unwrap();

    let token = env::var("DISCORD_TOKEN").unwrap();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(":"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token)
        .framework(framework)
        .event_handler(Handler)
        .await.unwrap();

    {
        let mut data = client.data.write().await;
        data.insert::<DatabaseConnection>(Mutex::new(conn));
    }

    client.start().await.unwrap();
}
