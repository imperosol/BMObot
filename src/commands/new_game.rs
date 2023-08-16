/// Archive la partie en cours (s'il y en a une) puis démarre une nouvelle partie.
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use crate::game_logic::GAME;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    GAME.lock().await.archive();
    GAME.lock().await.reset();
    command
        .create_interaction_response(&ctx.http, |res| {
            res.interaction_response_data(|msg| msg.content("Nouvelle partie créée".to_string()))
        })
        .await
        .unwrap();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("new_game")
        .description("Crée une nouvelle partie")
}
