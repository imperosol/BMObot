use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Mentionable;
use crate::discord_utils;
use crate::game_logic::GAME;


pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let user = command.data.resolved.members
        .keys().next().unwrap()
        .to_user(&ctx.http).await.unwrap();
    let member = command.data.guild_id.unwrap()
        .member(&ctx.http, user.id).await.unwrap();
    match GAME.lock().await.add_player(&user) {
        Ok(_) => {
            for role in ["joueur", "mage débutant"] {
                discord_utils::give_role(ctx, &member, role).await;
            }
            command.create_interaction_response(
                &ctx.http, |res| res.interaction_response_data(
                    |msg| msg.content(format!("Joueur ajouté : {}", user.mention()))
                ),
            ).await.unwrap();
        }
        Err(why) => command.create_interaction_response(
            &ctx.http, |res| res.interaction_response_data(
                |msg| msg.content(why)
            ),
        ).await.unwrap(),
    };
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("add_player")
        .description("Ajoute un joueur à la partie")
        .create_option(|option| {
            option
                .name("joueur")
                .description("Le joueur qui reçoit le paquet de cartes")
                .kind(CommandOptionType::User)
                .required(true)
        })
}