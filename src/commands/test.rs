use serenity::{client::Context, model::channel::Message};

pub async fn test(ctx: Context, msg: Message) {
    println!("{:?}", msg.guild_id.expect("Message was not sent in a guild"));

    println!("{:?}", msg.clone().member.unwrap().permissions);
    println!("{:?}", msg.clone().member(&ctx).await.unwrap().permissions);
    println!("{:?}", msg.member(&ctx).await.unwrap().permissions(&ctx));
    println!("{:?}", msg.guild(&ctx).expect("Failed to fetch guild").member(&ctx, &msg.author.id).await.expect("Failed to fetch member").permissions);
    println!("{:?}", msg.guild(&ctx).expect("Failed to fetch guild").member(&ctx, &msg.author.id).await.expect("Failed to fetch member").permissions(&ctx));

    msg.reply(&ctx, "Successfully ran tests").await.expect("Failed to send message");
}
