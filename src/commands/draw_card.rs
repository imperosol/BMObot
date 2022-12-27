use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction};
use serenity::model::user::User;
use serenity::prelude::{Context, Mentionable};
use crate::discord_utils::command_response;
use crate::game_logic::{GAME, MagicLevel};

async fn draw_card_beginner(user: &User, ctx: &Context, command: &ApplicationCommandInteraction) {
    println!("0");
    let Some(card) = GAME.lock().await.player_draw_cards(&user) else {
        command_response(ctx, command, format!("{} n'a plus de cartes", user.name)).await;
        return;
    };
    println!("1");
    let mut msg = format!("{} tire la carte :\n", user.mention());
    msg.push_str(format!("- {}\n", card).as_str());
    println!("2");
    let remaining = GAME.lock().await.player_remaining_cards(&user).unwrap();
    msg.push_str(format!("Cartes restantes : {}", remaining).as_str());
    println!("3");
    command_response(ctx, command, msg).await;
}


async fn draw_card_veteran(user: &User, ctx: &Context, command: &ApplicationCommandInteraction) {
    let Some(card) = GAME.lock().await.player_draw_cards(user) else {
        command_response(ctx, command, format!("{} n'a plus de cartes", user.name)).await;
        return;
    };
    let hand = GAME.lock().await.player_get_hand(user).unwrap().unwrap();
    let mut msg = format!("{} Quelle carte jouer ?\n", user.mention());
    msg.push_str(format!(":regional_indicator_a: {}\n", hand.get(0).unwrap()).as_str());
    msg.push_str(format!(":regional_indicator_b: {}\n", hand.get(1).unwrap()).as_str());
    msg.push_str(format!(":regional_indicator_c: {}\n", card).as_str());
    command_response(ctx, command, msg).await;
}

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let user = command.data.resolved.members
        .keys().next().unwrap()
        .to_user(&ctx.http).await.unwrap();
    if !GAME.lock().await.player_exists(&user) {
        command_response(ctx, command, format!("{} n'est pas un joueur", user.name)).await;
        return;
    };
    println!("yo");
    match GAME.lock().await.players.get(&user.id).unwrap().magic_level {
        MagicLevel::Beginner => {
            println!("beginner");
            draw_card_beginner(&user, ctx, command).await },
        MagicLevel::Veteran => draw_card_veteran(&user, ctx, command).await
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("draw_card")
        .description("Fait tirer des cartes au joueur")
        .create_option(|option| {
            option
                .name("joueur")
                .description("Le joueur qui reçoit le paquet de cartes")
                .kind(CommandOptionType::User)
                .required(true)
        })
}