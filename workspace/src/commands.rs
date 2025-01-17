use std::{str, time::Duration};
use anyhow::Result as AnyhowResult;
use serenity::all::{CreateEmbed, CreateEmbedAuthor, CreateMessage, Message};
use tokio::{sync::Mutex, time};

#[derive(Debug)]
pub struct Data {
    pub points: Mutex<u32>
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Responds with "world!"
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

// Add Player Points(Useable Guild Owner) ※実装中
#[poise::command(slash_command)]
pub async fn increase_point_to_user(
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
pub async fn slot_play(ctx: Context<'_>) -> Result<(),Error> {
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
pub async fn show_point(ctx: Context<'_>) -> Result<(),Error> {
    ctx.say(format!("{:#?}",ctx.data().points)).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn developer_access(ctx: Context<'_>) -> Result<(),Error> {
    if let Err(why) = ctx.channel_id().send_message(
        &ctx.http(),
        CreateMessage::new().add_embed(
            CreateEmbed::new()
                .author(CreateEmbedAuthor::new("泡沫 京水"))
                .url("https://x.com/utakatakyosui")
                .description("プログラミングﾁｮｯﾄﾃﾞｷﾙ大学生")
                .image("https://pbs.twimg.com/profile_images/1861649062606577665/GHbnyWIG_400x400.jpg")
            )
    )
    .await {
        eprintln!("エラー: {:?}",why);
    }
    Ok(())
}