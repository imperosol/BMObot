mod commands;
mod game_logic;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

const GUILD_ID: GuildId = GuildId(462189760727875586);
const BOT_TOKEN: &str = "NDU3OTM0OTEyMjU1ODE5Nzc3.WyaDHw.O_swwgXviX1IUbgUtHEURJr7Mak";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        GUILD_ID.set_application_commands(&ctx.http, |commands|
            commands
                .create_application_command(|command| commands::new_game::register(command))
                .create_application_command(|command| commands::create_deck::register(command))
        ).await.unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            if command.data.name.as_str() == "create_deck" {
                commands::create_deck::run(&command.data.options);
            }

            let content = "Bienvenue".to_string();

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut client = Client::builder(BOT_TOKEN, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .unwrap();

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}