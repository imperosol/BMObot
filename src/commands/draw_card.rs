use std::time::Duration;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction};
use serenity::model::user::User;
use serenity::prelude::{Context, Mentionable};
use crate::discord_utils::command_response;
use crate::game_logic::{GAME, MagicLevel};

pub async fn draw_card_beginner(user: &User, ctx: &Context, command: &ApplicationCommandInteraction) {
    let card = GAME.lock().await.player_draw_cards(user);
    let Some(card) = card else {
        command_response(ctx, command, format!("{} n'a plus de cartes", user.name)).await;
        return;
    };
    let mut msg = format!("{} tire la carte :\n", user.mention());
    msg.push_str(format!("- {}\n", card).as_str());
    let remaining = GAME.lock().await.player_remaining_cards(user).unwrap();
    msg.push_str(format!("Cartes restantes : {}", remaining).as_str());
    command_response(ctx, command, msg).await;
}

pub async fn draw_card_veteran(user: &User, ctx: &Context, command: &ApplicationCommandInteraction) {
    let Some(card) = GAME.lock().await.player_draw_cards(user) else {
        command_response(ctx, command, format!("{} n'a plus de cartes", user.name)).await;
        return;
    };
    command.create_interaction_response(&ctx.http, |res| {
        res.interaction_response_data(|data| {
            data.ephemeral(true);
            data.content(format!("{} a une minute pour choisir sa carte", user.name))
        })
    }).await.unwrap();
    let hand = GAME.lock().await.player_get_hand(user).unwrap().unwrap().clone();
    let card_choice_msg = command.channel_id.send_message(&ctx.http, |msg| {
        msg.content(format!("{}, choisissez une carte à jouer", user.mention()))
            .components(|c| {
                c.create_action_row(|row| {
                    row.create_select_menu(|menu| {
                        menu.custom_id("card select");
                        menu.placeholder("Choisissez une carte");
                        menu.options(|f| {
                            f.create_option(|o| o.label(&hand[0]).value("A"));
                            f.create_option(|o| o.label(&hand[1]).value("B"));
                            f.create_option(|o| o.label(&card).value("C"))
                        })
                    })
                })
            })
    }).await.unwrap();
    let Some(interaction) = card_choice_msg
        .await_component_interaction(ctx)
        .timeout(Duration::from_secs(60)).await else {
        card_choice_msg.reply(&ctx, "Timeout. Aucune action effectuée.").await.unwrap();
        return;
    };
    let card_id = &interaction.data.values[0];
    let card = match card_id.as_str() {
        "A" => {
            GAME.lock().await
                .player_set_hand(user, vec![hand[1].clone(), card])
                .unwrap();
            &hand[0]
        }
        "B" => {
            GAME.lock().await
                .player_set_hand(user, vec![hand[0].clone(), card])
                .unwrap();
            &hand[1]
        }
        "C" => &card,
        other => {
            eprintln!("Valeur de choix de carte non prévue : {}", other);
            return;
        }
    };
    card_choice_msg.delete(&ctx.http).await.unwrap();
    command.channel_id.send_message(&ctx.http, |msg| {
        msg.add_embed(|embed| {
            embed.description(format!("{} utilise la carte {}", user.mention(), card))
        })
    }).await.unwrap();
}

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let user = command.data.resolved.members
        .keys().next().unwrap()
        .to_user(&ctx.http).await.unwrap();
    let level = match GAME.lock().await.players.get(&user.id) {
        None => {
            command_response(ctx, command, format!("{} n'est pas un joueur", user.name)).await;
            return;
        }
        Some(player) => player.magic_level
    };
    match level {
        MagicLevel::Veteran => draw_card_veteran(&user, ctx, command).await,
        MagicLevel::Beginner => draw_card_beginner(&user, ctx, command).await,
    };
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