#[macro_use]
extern crate diesel;

mod commands;
mod schema;
mod models;
mod utils;

use std::{
    env,
    time::SystemTime,
    collections::HashMap,
};

use diesel::{
    prelude::*,
    sqlite::SqliteConnection,
};
use dotenv::dotenv;
use serenity::{
    async_trait,
    framework::standard::{
        CommandResult, StandardFramework,
        macros::{group, hook}
    },
    model::prelude::*,
    prelude::*
};
use tracing::{error, info};
use tracing_subscriber::{FmtSubscriber, EnvFilter};

use commands::{
    amimir::*,
    nickname::*,
    user::*,
    uptime::*,
    edt::*,
};

struct DatabaseConnection;

impl TypeMapKey for DatabaseConnection {
    type Value = Mutex<SqliteConnection>;
}

struct Uptime;

impl TypeMapKey for Uptime {
    type Value = SystemTime;
}

struct NicknameTimeout;

impl TypeMapKey for NicknameTimeout {
    type Value = Mutex<HashMap<UserId, SystemTime>>;
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

#[hook]
#[instrument]
async fn before_hook(_ctx: &Context, msg: &Message, cmd_name: &str) -> bool {
    info!("got command {} from {}", cmd_name, msg.author);
    true
}

#[hook]
async fn after_hook(_ctx: &Context, _msg: &Message, cmd_name: &str, error: CommandResult) {
    if let Err(err) = error {
        error!("in {}: {}", cmd_name, err);
    }
}

#[group]
#[commands(amimir, nickname, user, uptime, edt)]
struct General;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set the global defaut subscriber");

    let db_url = env::var("DATABASE_URL")
        .expect("failed to get the database url/path");
    let conn = SqliteConnection::establish(&db_url)
        .expect("failed to establish connection with SQLite");

    let token = env::var("DISCORD_TOKEN")
        .expect("failed to get the Discord token");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(":"))
        .before(before_hook)
        .after(after_hook)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("client builder failed");

    {
        let mut data = client.data.write().await;
        data.insert::<DatabaseConnection>(Mutex::new(conn));
        data.insert::<Uptime>(SystemTime::now());
        data.insert::<NicknameTimeout>(Mutex::new(HashMap::new()));
    }

    client.start().await
        .expect("client failed");
}
