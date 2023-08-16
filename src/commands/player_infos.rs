use crate::discord_utils::command_response;
use crate::game_logic::GAME;
/// Affiche les infos sur un joueur et les cartes qu'il a en main si c'est un mage intermédiaire.
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let user = command
        .data
        .resolved
        .members
        .keys()
        .next()
        .unwrap()
        .to_user(&ctx.http)
        .await
        .unwrap();
    if !GAME.lock().await.player_exists(&user) {
        command_response(ctx, command, format!("{} n'est pas un joueur", user.name)).await;
        return;
    };
    let res = GAME.lock().await.get_player_string(&user).unwrap();
    command_response(ctx, command, res).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("player_infos")
        .description("Montre les infos sur un joueur")
        .create_option(|option| {
            option
                .name("joueur")
                .description("Le joueur dont on veut le détail")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
