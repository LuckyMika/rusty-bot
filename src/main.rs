mod commands;
mod config;
mod utils;

use std::{collections::{HashMap, VecDeque}, sync::{Arc, Mutex}};
use commands::{ping::ping, shutdown::shutdown, new::new, config::config, test::test};
use config::ticket::{TicketConfig};
use serenity::{futures::future::BoxFuture, prelude::TypeMapKey, client::bridge::gateway::ShardManager};
use serenity::{Client, model::channel::Message, model::gateway::Ready, prelude::{GatewayIntents, Context, EventHandler}, async_trait};

type CommandFunction = Arc<dyn Fn(Context, Message) -> BoxFuture<'static, ()> + Send + Sync>;

static PREFIX: &str = "$";

pub struct ShardManagerContainer;

struct TicketConfigContainer;

impl TypeMapKey for TicketConfigContainer {
    type Value = Arc<Mutex<TicketConfig>>;
}

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler {
    commands: Arc<Mutex<HashMap<String, CommandFunction>>>,
}

impl Handler {
    fn new() -> Handler {
        return Handler {
            commands: Arc::new(Mutex::new(HashMap::new()))
        }
    }


    fn register_command(&mut self, command_name: String, handler_function: CommandFunction) {
        let mut commands = self.commands.lock().expect("Command map locking failed");
        commands.insert(command_name, handler_function);
    }

    async fn execute(&self, command_name: &str, ctx: Context, msg: Message) {

        let func = self.commands.lock().unwrap().get(command_name).cloned();
        if let Some(func) = func {
                func(ctx, msg).await;
        }
    }
}


#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if !message.content.starts_with("$") {
            return
        }

        let mut arguments: VecDeque<&str> = message.content.split_whitespace().collect();
        let first = arguments.pop_front().expect("Failed to pop front element.")[PREFIX.len()..].to_string();

        self.execute(&first[..], ctx, message).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name)
    }
}





#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load environment.");
    let token = dotenv::var("TOKEN").expect("Did not receive token in environment.");

    let ticket_config = TicketConfig::load();

    let mut event_handler = Handler::new();

    event_handler.register_command("hello".to_string(), Arc::new(|ctx, msg| {
        Box::pin(async move {
            if let Err(why) = msg.reply(&ctx, "Hello from rust!").await {
                println!("Error whilst executing command 'hello': {}", why);
            }
        })
    }));

    event_handler.register_command("ping".to_string(), Arc::new(|ctx, msg| {
        Box::pin(async move {
            ping(ctx, msg).await;
        })
    }));

    event_handler.register_command("shutdown".to_string(), Arc::new(|ctx, msg| {
        Box::pin(async move {
            shutdown(ctx, msg).await;
        })
    }));
    

    event_handler.register_command("new".to_string(), Arc::new(|ctx, msg| {
        Box::pin(async move {
            new(ctx, msg).await;
        })
    }));

    event_handler.register_command("config".to_string(), Arc::new(|ctx, msg| {
        Box::pin(async move {
            config(ctx, msg).await;
        })
    }));

    event_handler.register_command("test".to_string(), Arc::new(|ctx, msg| {
        Box::pin(async move {
            test(ctx, msg).await;
        })
    }));
    let mut client = Client::builder(&token, GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MEMBERS)
        .event_handler(event_handler)
        .await.unwrap();
    {
        let mut data = client.data.write().await;
        data.insert::<TicketConfigContainer>(Arc::new(Mutex::new(ticket_config)));
    }
    if let Err(why) = client.start().await {
        println!("Error starting client: {}", why)
    }
}
