use std::time::Instant;

use serenity::prelude::Context;
use serenity::model::channel::Message;


pub async fn ping(ctx: Context, msg: Message) {


    match  measure_api_latency(&ctx).await {
        Ok(latency) => {
            if let Err(why) = msg.reply(&ctx, format!("My ping is {}ms", latency)).await {
                println!("Error sending message: {}", why);
            }
        },
        Err(error) => {
            if let Err(why) = msg.reply(&ctx, "Failed to get latency!").await {
                println!("Error sending message: {}", why);
            }
            println!("Error getting latency: {}", error)
        }
    }
}

async fn measure_api_latency(ctx: &Context) -> Result<u128, serenity::Error> {
    let http = &ctx.http;

    let start = Instant::now();
    if let Err(err) = http.get_current_user().await {
        return Err(err);
    }
    let end = Instant::now();

    let latency = (end - start).as_millis();
    return Ok(latency);
}