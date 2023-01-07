use serenity::{
    all::{OnlineStatus, Ready},
    gateway::ActivityData,
};

use crate::{dev_guild, error, info, prelude::*, DEV_BUILD};

#[derive(Debug)]
pub struct Events {
    pub logger: Logger,
}

impl Events {
    pub const fn new(logger: Logger) -> Self {
        Self { logger }
    }

    pub async fn create_commands(&self, http: &Http) -> Result<()> {
        let guild_id = dev_guild()?;
        let cmds = vec![data::new(), help::new(), ping::new()];

        let global = if DEV_BUILD {
            http.get_global_application_commands().await?.len()
        } else {
            http.create_global_application_commands(&cmds).await?.len()
        };

        info!(self.logger, "Created {global} global commands");

        let guild = guild_id.set_application_commands(http, cmds).await?.len();

        info!(self.logger, "Created {guild} guild commands");

        Ok(())
    }
}

#[async_trait]
impl EventHandler for Events {
    async fn ready(&self, context: Context, ready: Ready) {
        info!(self.logger, "Connected as \"{}\"", ready.user.tag());

        if let Some(count) = ready.shard.map(|s| s.total) {
            info!(self.logger, "Using {count} shards");
        }

        context.set_presence(Some(ActivityData::listening("/help")), OnlineStatus::Idle);

        if let Err(error) = self.create_commands(context.http()).await {
            let time = Local::now();

            error!(self.logger, time, "Error creating commands: {error}");
        }
    }

    // async fn message(&self, context: Context, message: Message) {}

    #[allow(clippy::match_wildcard_for_single_variants)]
    async fn interaction_create(&self, context: Context, mut interaction: Interaction) {
        let http = context.http();
        let id = match &interaction {
            Interaction::Autocomplete(i) => format!("{}<acp:{}>", i.data.name, i.id),
            Interaction::Command(i) => format!("{}<cmd:{}>", i.data.name, i.id),
            Interaction::Component(i) => format!("{}<cpn:{}>", i.data.custom_id, i.id),
            Interaction::Modal(i) => format!("{}<mdl:{}>", i.data.custom_id, i.id),
            Interaction::Ping(i) => format!("{}<png:{}>", i.token, i.id),
        };

        let result: Result<()> = match &mut interaction {
            Interaction::Command(command) => match command.data.name.as_str() {
                data::NAME => data::command(&context, command).await,
                help::NAME => help::command(&context, command).await,
                ping::NAME => ping::command(&context, command).await,
                _ => Err(anyhow!("unknown interaction: {id}")),
            },
            _ => Err(anyhow!("unknown interaction: {id}")),
        };

        if let Err(error) = result {
            let time = Local::now();
            let code = Logger::error_code(time);

            error!(self.logger, time, "Interaction failed: {id} - {error}");

            let embed = CreateEmbed::new()
                .color(Color::RED)
                .description(format!("> {error}\n\nError code: `{code}`"))
                .title("An error occurred!");
            let response = CreateInteractionResponseFollowup::new()
                .embed(embed)
                .ephemeral(true);

            let result = match &interaction {
                Interaction::Autocomplete(i) => {
                    i.create_followup(http, response).await.map_err(Into::into)
                }
                Interaction::Command(i) => {
                    i.create_followup(http, response).await.map_err(Into::into)
                }
                Interaction::Component(i) => {
                    i.create_followup(http, response).await.map_err(Into::into)
                }
                Interaction::Modal(i) => {
                    i.create_followup(http, response).await.map_err(Into::into)
                }
                i => Err(anyhow!("invalid interaction type: {:?}", i.kind())),
            };

            if let Err(error) = result {
                info!(self.logger, "Error was not displayed: ({code}) {error}");
            }
        } else {
            info!(self.logger, "Interaction succeeded: {id}");
        }
    }
}
