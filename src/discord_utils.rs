use serenity::client::Context;
use serenity::model::guild::Member;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub async fn give_role(ctx: &Context, member: &Member, role_name: &str) {
    let guild_id = member.guild_id;
    let role = guild_id.roles(&ctx.http).await.unwrap()
        .values()
        .find(|r| r.name == role_name)
        .map(|r| r.id);
    let role = match role {
        Some(r) => r,
        None => member.guild_id.create_role(
            &ctx.http, |r| r.name(role_name),
        ).await.unwrap().id
    };
    if !member.roles.contains(&role) {
        let mut member = guild_id
            .member(&ctx.http, member.user.id)
            .await.unwrap();
        member.add_role(&ctx.http, role).await.unwrap();
    }
}

pub async fn command_response<T: ToString>(ctx: &Context, command: &ApplicationCommandInteraction, message: T) {
    command.create_interaction_response(
        &ctx.http, |res| res.interaction_response_data(
            |msg| msg.content(message)
        ),
    ).await.unwrap();
}
