mod commands;

use tokio::sync::Mutex;

use anyhow::Context as _;
use commands::*;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use serenity::{all::EventHandler, async_trait};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;



#[allow(unused)]
struct Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: poise::serenity_prelude::Context, ready: poise::serenity_prelude::Ready) {
        println!("{} が起動しました\n", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(),slot_play(),show_point(),developer_access()],
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
                    points: Mutex::new(500)
                })
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
