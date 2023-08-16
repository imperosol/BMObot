use once_cell::sync::Lazy;
use serenity::client::Context;
use serenity::model::guild::Member;
use serenity::model::id::{GuildId, RoleId};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use std::env;

pub static GUILD_ID: Lazy<GuildId> = Lazy::new(|| {
    GuildId(
        env::var("GUILD_ID")
            .expect("Il faut renseigner l'id du serveur dans le .env")
            .parse()
            .expect("L'id du serveur dans le .env n'est pas dans un format valide"),
    )
});

pub async fn give_role(ctx: &Context, member: &mut Member, role_name: &str) {
    let role = GUILD_ID
        .roles(&ctx.http)
        .await
        .unwrap()
        .values()
        .find(|r| r.name == role_name)
        .map(|r| r.id);
    let role = match role {
        Some(r) => r,
        None => {
            GUILD_ID
                .create_role(&ctx.http, |r| r.name(role_name))
                .await
                .unwrap()
                .id
        }
    };
    if !member.roles.contains(&role) {
        member.add_role(&ctx.http, role).await.unwrap();
    }
}

pub async fn clear_roles(ctx: &Context, member: &mut Member) {
    let to_delete = GUILD_ID
        .roles(&ctx.http)
        .await
        .unwrap()
        .values()
        .filter(|role| {
            ["joueur", "mage débutant", "mage intermédiaire"].contains(&role.name.as_str())
        })
        .filter(|role| member.roles.contains(&role.id))
        .map(|role| role.id)
        .collect::<Vec<RoleId>>();
    member
        .remove_roles(&ctx.http, &to_delete)
        .await
        .expect("Couldn't delete roles");
}

pub async fn command_response<T: ToString>(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    message: T,
) {
    command
        .create_interaction_response(&ctx.http, |res| {
            res.interaction_response_data(|msg| msg.embed(|embed| embed.description(message)))
        })
        .await
        .unwrap();
}
