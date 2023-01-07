use crate::prelude::*;

pub const NAME: &str = "ping";

pub fn new() -> CreateCommand {
    CreateCommand::new(NAME)
        .default_member_permissions(Permissions::USE_APPLICATION_COMMANDS)
        .description("Calculates the bot's API response time")
        .dm_permission(true)
}

pub async fn command(context: &Context, command: &CommandInteraction) -> Result<()> {
    command.defer_ephemeral(context).await?;

    let bot_user = context.http().get_current_user().await?;
    let author = CreateEmbedAuthor::new(bot_user.tag()).icon_url(bot_user.face());
    let color = bot_user.accent_colour.unwrap_or(Color::ROSEWATER);
    let mut embed = CreateEmbed::new()
        .author(author)
        .color(color)
        .title("Calculating...");

    let builder = CreateInteractionResponseFollowup::new().embed(embed.clone());
    let message = command.create_followup(context, builder).await?;

    let sent = message.id.created_at().timestamp_millis();
    let received = command.id.created_at().timestamp_millis();
    let ms = sent - received;
    embed = embed.title(format!("Pong! ({ms}ms)"));

    let builder = CreateInteractionResponseFollowup::new().embed(embed);
    command.edit_followup(context, message.id, builder).await?;

    Ok(())
}
