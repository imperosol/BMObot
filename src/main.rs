mod commands;
mod discord_utils;
mod files;
mod game_logic;

use crate::discord_utils::GUILD_ID;
use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        GUILD_ID
            .set_application_commands(&ctx.http, |commands| {
                commands
                    .create_application_command(|command| commands::new_game::register(command))
                    .create_application_command(|command| commands::add_player::register(command))
                    .create_application_command(|command| commands::draw_card::register(command))
                    .create_application_command(|command| commands::promote::register(command))
                    .create_application_command(|command| commands::player_infos::register(command))
            })
            .await
            .unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Commande reçue : {}", command.data.name);

            match command.data.name.as_str() {
                "new_game" => commands::new_game::run(&ctx, &command).await,
                "add_player" => commands::add_player::run(&ctx, &command).await,
                "draw_card" => commands::draw_card::run(&ctx, &command).await,
                "promote" => commands::promote::run(&ctx, &command).await,
                "player_infos" => commands::player_infos::run(&ctx, &command).await,
                _ => println!("Cette commande n'existe pas ou appartient à un autre bot"),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Le token du bot doit être renseigné dans le .env");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .unwrap();

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
