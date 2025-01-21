mod commands;
mod utils;
mod entities;
mod migration;
mod cron;


use std::str::FromStr;

use apalis::{layers::{retry::RetryPolicy, WorkerBuilderExt}, prelude::{WorkerBuilder, WorkerFactoryFn}};
use cron::dairy_charge::dairy_charge;
use sea_orm::SqlxPostgresConnector;
use sea_orm_migration::prelude::*;
use tokio::sync::Mutex;
use crate::migration::migrator::Migrator;
use anyhow::Context as _;
use commands::*;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use serenity::{all::EventHandler, async_trait};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use apalis_cron::{CronStream, Schedule};


#[allow(unused)]
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: poise::serenity_prelude::Context, ready: poise::serenity_prelude::Ready) {
        println!("{} が起動しました\n", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://utakata:1336@postgres:5432/utakata_db"
    )] pool: sqlx::PgPool ,
) -> ShuttleSerenity {
    
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let connection = SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone());  
    Migrator::up(&connection, None).await.expect("Migration Error");
    

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(),slot_play(),show_point(),developer_access(),gift_point(),check_emoji()],
            event_handler: |#[allow(unused)] ctx,#[allow(unused)] event,#[allow(unused)] f_ctx,#[allow(unused)] data| {
                Box::pin(async {
                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    db: Mutex::from(connection)
                })
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    /* let schedule = Schedule::from_str("@daily").unwrap();
    WorkerBuilder::new("dairy-charge")
        .retry(RetryPolicy::retries(5))
        .data(connection.clone())
        .backend(CronStream::new(schedule))
        .build_fn(dairy_charge)
        .run().await; */
        
    Ok(client.into())
}
