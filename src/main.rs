mod commands;
mod game_logic;
mod discord_utils;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use dotenv::dotenv;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Il faut renseigner l'id du serveur dans le .env")
                .parse().unwrap()
        );
        guild_id.set_application_commands(&ctx.http, |commands|
            commands
                .create_application_command(|command| commands::new_game::register(command))
                .create_application_command(|command| commands::add_player::register(command))
                .create_application_command(|command| commands::draw_card::register(command))
                .create_application_command(|command| commands::promote::register(command))
                .create_application_command(|command| commands::player_infos::register(command))
        ).await.unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {}", command.data.name);

            match command.data.name.as_str() {
                "new_game" => commands::new_game::run(&ctx, &command).await,
                "add_player" => commands::add_player::run(&ctx, &command).await,
                "draw_card" => commands::draw_card::run(&ctx, &command).await,
                "promote" => commands::promote::run(&ctx, &command).await,
                "player_infos" => commands::player_infos::run(&ctx, &command).await,
                _ => println!("Cette commande n'existe pas")
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .unwrap();

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}