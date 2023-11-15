use serenity::{prelude::Context, model::{channel::Message, guild::Guild, prelude::{ChannelCategory, ChannelId}, Permissions}};

use crate::{TicketConfigContainer, utils::parsers::parse_category_from_name};

pub async fn config(ctx: Context, msg: Message) {
    let mut args = msg.content.split_whitespace().skip(1).collect::<Vec<&str>>();
    let data = ctx.data.read().await;

    let config = match data.get::<TicketConfigContainer>() {
        Some(config) => Some(config),
        None => {
            msg.reply(&ctx, "Failed to retrieve the configuration").await.expect("Failed to send message");

            None
        }
    };

    if config.is_none() {
        return;
    }

    let admin_roles = config.unwrap().lock().expect("Failed to get lock").admin_roles.clone();
    let mut is_ticket_admin = false;

    for role in msg.member(&ctx).await.expect("Failed to get member").roles {
        if admin_roles.contains(&role) {
            is_ticket_admin = true;
            break;
        }
    }
    let is_server_admin = msg.guild_id.expect("Failed to find guild")
        .to_partial_guild(&ctx).await.expect("Failed to query guild")
        .member(&ctx, &msg.member(&ctx).await.expect("Failed to get member from message")).await.expect("Failed to find member")
        .permissions(&ctx).expect("Failed to get member permissions")
        .contains(Permissions::ADMINISTRATOR);

    if !is_ticket_admin && !is_server_admin {
        msg.reply(&ctx, "You do not have permission to use this command.").await.expect("Failed to send message");
        return;
    }

    if args.is_empty() {
        msg.reply(&ctx, "Usage: config <category|support|admin>").await.expect("Failed to send message"); //TODO: Make proper usage
        return;
    }
    
    let command = args.remove(0);
    if command == "category" {
        if args.is_empty() {
            
            let data = ctx.data.read().await;

            let config = match data.get::<TicketConfigContainer>() {
                Some(config) => Some(config),
                None => {
                    msg.reply(&ctx, "Failed to retrieve the configuration").await.expect("Failed to send message");

                    None
                }
            };

            if config.is_none() {
                return;
            }
            let category_id = config.unwrap().lock().expect("Failed to acquire lock on config").category;
            let category_name = category_id.clone().name(&ctx).await;

            if category_name.is_none() {
                msg.reply(&ctx, "You currently do not have a category configured.").await.expect("Failed to send message");

                return;
            }

            msg.reply(&ctx, format!("Your current ticket category is `{}`", category_name.unwrap())).await.expect("Failed to send message");
        } else {
            let arg = args.join(" ");

            match get_channel(&msg.guild(&ctx).unwrap(), &arg) {
                Some(channel) => {
                    let mut data = ctx.data.write().await;

                    let config = match data.get::<TicketConfigContainer>() {
                        Some(config) => Some(config),
                        None => {
                            msg.reply(&ctx, "Failed to retrieve the configuration").await.expect("Failed to send message");

                            None
                        }
                    };
                    

                    if config.is_none() {
                        return;
                    }

                    let config = config.unwrap().clone();
                    let failed; 
                    {
                        let mut thing = config.lock().expect("Failed to acquire lock on config");

                        thing.category = channel.id;
                        failed = match thing.save() {
                            Ok(()) => false,
                            Err(why) => {
                                println!("Failed to save config: {}", why);
                                true
                            }
                        };
                    }

                    if failed {
                        msg.reply(&ctx, "Failed to save the configuration.").await.expect("Failed to send message");
                        return;
                    }

                    data.insert::<TicketConfigContainer>(config.clone());

                    msg.reply(&ctx, format!("Your ticket category has been set to `{}`", channel.name)).await.expect("Failed to send message"); 
                },
                None => {
                    msg.reply(&ctx, format!("Could not find a category with the name or id of `{}`", &arg)).await.expect("Failed to send message"); 
                }
            }
        }
    } else if command == "support" {
        msg.reply(&ctx, "NOT IMPLEMENTED").await.expect("Failed to send message");
    } else if command == "admin" {
        msg.reply(&ctx, "NOT IMPLEMENTED").await.expect("Failed to send message");
    } else {
        msg.reply(&ctx, "Usage: config <category|support|admin>").await.expect("Failed to send message"); //TODO: Make proper usage
        return;
    }
}

fn get_channel(guild: &Guild, name_or_id: &str) -> Option<ChannelCategory> {
    if let Ok(id) = name_or_id.parse::<u64>() {
        let channel = guild.channels.get(&ChannelId(id));

        if channel.is_none() {
            return None
        }

        return channel.unwrap().clone().category()
    } else {
        parse_category_from_name(guild, name_or_id.to_owned())
    }
}
