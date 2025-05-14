use serenity::all::{
    ChannelType, CommandInteraction, Context, CreateCommand, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, Permissions,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let guild_id = interaction.guild_id.unwrap();

    let guild = ctx.http.get_guild(guild_id).await?;

    let owner_id = guild.owner_id.get();
    let roles_count = guild.roles.len().to_string();
    let members = guild.members(&ctx.http, None, None).await.unwrap();

    let members_count = members.iter().filter(|m| !m.user.bot).count().to_string();

    let channels_count = guild
        .channels(&ctx.http)
        .await
        .unwrap()
        .values()
        .filter(|c| c.kind != ChannelType::Category)
        .count()
        .to_string();

    let embed = CreateEmbed::new()
        .title(format!("Server Information of {}", guild.name))
        .field("Members", format!("``{members_count}``"), true)
        .field("Roles", format!("``{roles_count}``"), true)
        .field("Channels", format!("``{channels_count}``"), true)
        .field("Owner", format!("<@{}>", owner_id), true)
        .thumbnail(guild.icon_url().unwrap_or_default())
        .color(0xFF69B4);

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed),
            ),
        )
        .await
        .expect("Failed to create response");

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("server_info")
        .description("Get information about the server")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .dm_permission(false)
}
