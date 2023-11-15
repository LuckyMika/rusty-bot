use serenity::prelude::Context;
use serenity::model::channel::Message;

pub async fn shutdown(ctx: Context, msg: Message) {
    if !msg.author.id.0 == 387344884559773699 {
        if let Err(why) = msg.reply(&ctx, "You don't have permission to execute this command!").await {
            println!("Error sending message: {}", why);
        }
        return;
    }

    println!("Global shutdown command called!");
    if let Err(why) = msg.reply(&ctx, "Shutting down...").await {
        println!("Error sending message: {}", why);
    }

    ctx.shard.shutdown_clean();

    std::process::exit(0);
}