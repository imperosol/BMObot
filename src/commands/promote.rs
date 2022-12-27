use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction};
use serenity::prelude::{Context, Mentionable};
use crate::discord_utils::command_response;
use crate::game_logic::{GAME, PromoteError};

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let user = command.data.resolved.members
        .keys().next().unwrap()
        .to_user(&ctx.http).await.unwrap();
    if !GAME.lock().await.player_exists(&user) {
        command_response(ctx, command, format!("{} n'est pas un joueur", user.name)).await;
        return;
    };
    if let Err(error) = GAME.lock().await.promote_player(&user).unwrap() {
        match error {
            PromoteError::AlreadyPromoted => {
                command_response(ctx, command, format!("{} est déjà un mage intermédiaire", user.name)).await;
            }
            PromoteError::BecomeInsane => {
                command_response(ctx, command, format!("{} n'a plus de cartes et devient fou", user.name)).await;
            }
        }
        return;
    }
    let mut msg = format!("{} devient mage intermédiaire.", user.mention());
    let hand = GAME.lock().await.player_get_hand(&user);
    let hand = hand.unwrap();
    if let Some(hand) = hand {
        msg.push_str(" Il prend en main les cartes suivantes :\n");
        for card in hand {
            msg.push_str(format!("- {}\n", card).as_str());
        }
    }
    command_response(ctx, command, msg).await;

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("promote")
        .description("Fait passer un joueur au niveau de magie supérieur")
        .create_option(|option| {
            option
                .name("joueur")
                .description("Le joueur à promouvoir")
                .kind(CommandOptionType::User)
                .required(true)
        })
}