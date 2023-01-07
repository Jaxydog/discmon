use crate::prelude::*;

pub const NAME: &str = "data";
pub const STATEMENT: &str = include_str!("../../include/data/statement.txt");

pub fn new() -> CreateCommand {
    CreateCommand::new(NAME)
        .default_member_permissions(Permissions::USE_APPLICATION_COMMANDS)
        .description("Displays a statement about data usage and privacy")
        .dm_permission(true)
}

pub async fn command(context: &Context, command: &CommandInteraction) -> Result<()> {
    command.defer_ephemeral(context).await?;

    let bot_user = context.http().get_current_user().await?;
    let author = CreateEmbedAuthor::new(bot_user.tag()).icon_url(bot_user.face());
    let color = bot_user.accent_colour.unwrap_or(Color::ROSEWATER);
    let embed = CreateEmbed::new()
        .author(author)
        .color(color)
        .description(STATEMENT)
        .title("Data Usage and Privacy");

    let builder = CreateInteractionResponseFollowup::new().embed(embed);
    command.create_followup(context, builder).await?;

    Ok(())
}
