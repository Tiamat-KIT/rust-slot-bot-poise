use std::{str, time::Duration};

use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use serenity::{all::EventHandler, async_trait, futures::lock::Mutex};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use tokio::time;

struct Data {
    points: Mutex<u32>
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[allow(unused)]
struct Handler;

/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

// Add Player Points(Useable Guild Owner) ※実装中
#[poise::command(slash_command)]
async fn increase_point_to_user(
    ctx: Context<'_>,
    #[description = "Select User"] _user: Option<poise::serenity_prelude::User>,
    #[description = "Add Point Val"]  _point: u32
) -> Result<(), Error>{ 
    ctx.say("正直これどうしたらいいかわかってないぞ...ごめんな...。").await?;
    /* if point > 1000 {
        ctx.say("あげすぎるとめちゃくちゃ打たれるゾ...").await?;
    } */

    Ok(())
}

// play slot game
#[poise::command(slash_command)]
async fn slot_play(ctx: Context<'_>) -> Result<(),Error> {
    use rand::seq::SliceRandom;

    ctx.defer().await?;

    let mut point = ctx.data().points.lock().await;
    if *point < 10 {
        ctx.say("ポイントが足りません！貯めてから再チャレンジしてね！").await?;
        return Ok(())
    }

    *point -= 10;
    let mut slot_emojis:Vec<&str> = emojis::iter()
        .map(|e| e.as_str())
        .take(10)
        .cycle()
        .take(10 * 3)
        .collect();
    slot_emojis.shuffle(&mut rand::thread_rng());

    let [first,second,third] = slot_emojis[..3]
        .try_into()
        .unwrap();


    let slot_reach_first_result = first == second;
    let slot_reach_second_result = second == third;

    if slot_reach_first_result {
        ctx.say(format!("{} {} リーチ！ 【リーチボーナス +5pt】",first,second)).await?;
        *point += 5;
    }

    if slot_reach_second_result {
        ctx.say(format!("{} {} リーチ！ 【リーチボーナス +5pt】 ",second,third)).await?;
        *point += 5;
    }

    time::sleep(Duration::new(3, 0)).await;

    let slot_result = slot_reach_first_result && slot_reach_first_result;
    let result = if slot_result {
        format!("{} {} {} 揃いました！嬉しいね！",first,second,third)
    }else {
        format!("{} {} {} 揃わなかった...おつらいね...",first,second,third)
    };

    if slot_result {
        *point += 20;
    }
    ctx.say(result).await?;

    Ok(())
}

// show your points
#[poise::command(slash_command)]
async fn show_point(ctx: Context<'_>) -> Result<(),Error> {
    ctx.say(format!("{:#?}",ctx.data().points.try_lock().expect("エラー"))).await?;
    Ok(())
}

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
            commands: vec![hello(),slot_play(),show_point()],
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
                    points: Mutex::new(100)
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
