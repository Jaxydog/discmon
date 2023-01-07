use crate::prelude::*;

pub const NAME: &str = "help";
pub const HEADER: &str = include_str!("../../include/help/header.txt");
pub const FOOTER: &str = include_str!("../../include/help/footer.txt");

pub fn new() -> CreateCommand {
    CreateCommand::new(NAME)
        .default_member_permissions(Permissions::USE_APPLICATION_COMMANDS)
        .description("Displays a list of bot commands")
        .dm_permission(true)
}

pub async fn command(context: &Context, command: &CommandInteraction) -> Result<()> {
    command.defer_ephemeral(context).await?;

    let mut description = HEADER.to_string();

    if let Some(guild_id) = command.guild_id {
        let commands = guild_id.get_application_commands(context).await?;

        description.push_str("\n\n__Below is a list of available guild commands:__\n");

        if commands.is_empty() {
            description.push_str("> Looks like there aren't any available commands!");
        } else {
            description.push_str(&stringify(commands).join("\n"));
        }
    }

    let commands = context.http().get_global_application_commands().await?;

    description.push_str("\n\n__Below is a list of available global commands:__\n");

    if commands.is_empty() {
        description.push_str("> Looks like there aren't any available commands!");
    } else {
        description.push_str(&stringify(commands).join("\n"));
    }

    description.push_str(&format!("\n\n{FOOTER}"));

    let bot_user = context.http().get_current_user().await?;
    let author = CreateEmbedAuthor::new(bot_user.tag()).icon_url(bot_user.face());
    let color = bot_user.accent_colour.unwrap_or(Color::ROSEWATER);
    let embed = CreateEmbed::new()
        .author(author)
        .color(color)
        .description(description)
        .title("Hello, and welcome to DiscMon!");

    let builder = CreateInteractionResponseFollowup::new().embed(embed);
    command.create_followup(context, builder).await?;

    Ok(())
}

fn stringify(commands: Vec<Command>) -> Vec<String> {
    commands
        .into_iter()
        .map(|command| {
            let has_subcommands = command.options.iter().any(|o| {
                o.kind == CommandOptionType::SubCommand
                    || o.kind == CommandOptionType::SubCommandGroup
            });

            format!(
                "{bl}/{n}:{i}{br} - {d}{f1}{f2}",
                i = command.id,
                n = command.name,
                d = command.description,
                bl = if has_subcommands { '`' } else { '<' },
                br = if has_subcommands { '`' } else { '>' },
                f1 = if command.dm_permission.unwrap_or_default() {
                    "\n> *Command allows use in DMs*"
                } else {
                    ""
                },
                f2 = if has_subcommands {
                    "\n> *Command contains subcommands*"
                } else {
                    ""
                },
            )
        })
        .collect()
}
