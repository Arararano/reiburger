mod support;

use std::path::PathBuf;
use std::time::Duration;

use anyhow::anyhow;
use serenity::model::Timestamp;
use serenity::{async_trait};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};



struct Bot {
    path: PathBuf,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {

        if msg.content == "test" {
            if let Err(e) = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("soo genius?")
                .add_file("")   

            }).await {
                    error!("Error sending message: {:?}", e);
                }
        }


        if msg.content.to_lowercase() == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }
        if msg.content == "!kill yourself" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "I will destroy the world!").await {
                error!("Error sending message: {:?}", e);
            }
        }
        if msg.content == "are we balling?" {
            if let Err(e) = msg.channel_id.say(&ctx.http, support::get_time()).await {
                error!("Error sending message: {:?}", e);
            }
        }
        if msg.content == "surely" {
            if let Err(e) = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("soooo genius??")

            }).await {error!("Error sending message: {:?}", e);}
            
            if let Err(e) = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("https://media.entertainmentearth.com/assets/images/1dcf9f7c88944f6ebfde19cfca135e90xl.jpg")

            }).await {error!("Error sending message: {:?}", e);}
            
        }

        if msg.content == "kys" {
            
            if let Err(e) = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("https://cdn.discordapp.com/attachments/1126374891277537355/1126972321861734562/IMG_8629.png")

            }).await {
                error!("Error sending message: {:?}", e);
            }
            
            std::thread::sleep(Duration::from_millis(3000));            
            
            if let Err(e) = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("is this what you wanted?")

            }).await {
                error!("Error sending message: {:?}", e);
            }
            
        }

        

    


    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_static_folder::StaticFolder] static_f: PathBuf,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot { path: static_f})
        .await
        .expect("Err creating client");

    Ok(client.into())
}
