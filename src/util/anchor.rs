use crate::prelude::*;

pub trait TryAsAnchor {
    fn try_as_anchor(&self) -> Result<Anchor>;

    fn is_anchored(&self) -> bool {
        self.try_as_anchor().is_ok()
    }
    fn is_floating(&self) -> bool {
        self.try_as_anchor().is_err()
    }
}

pub trait AsAnchor {
    fn as_anchor(&self) -> Anchor;
}

impl<T: AsAnchor> TryAsAnchor for T {
    fn try_as_anchor(&self) -> Result<Anchor> {
        Ok(self.as_anchor())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Anchor {
    pub guild_id: Option<GuildId>,
    pub channel_id: ChannelId,
    pub message_id: MessageId,
}

impl Anchor {
    const URL: &str = "https://discord.com/channels";

    pub const fn new(
        guild_id: Option<GuildId>,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> Self {
        Self {
            guild_id,
            channel_id,
            message_id,
        }
    }
    pub const fn new_private(channel_id: ChannelId, message_id: MessageId) -> Self {
        Self::new(None, channel_id, message_id)
    }
    pub const fn new_guild(
        guild_id: GuildId,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> Self {
        Self::new(Some(guild_id), channel_id, message_id)
    }

    pub fn link(self) -> String {
        let mut url = Self::URL.to_string();

        if let Some(guild_id) = self.guild_id {
            url.push_str(&format!("/{guild_id}"));
        }

        format!("{url}/{}/{}", self.channel_id, self.message_id)
    }
    pub async fn to_partial_guild(self, cache_http: &impl CacheHttp) -> Result<PartialGuild> {
        self.guild_id
            .ok_or_else(|| anyhow!("missing guild identifier"))?
            .to_partial_guild(cache_http)
            .await
            .map_err(Into::into)
    }
    pub async fn to_guild_channel(self, cache_http: &impl CacheHttp) -> Result<GuildChannel> {
        let guild = self.to_partial_guild(cache_http).await?;
        let mut list = guild.channels(cache_http.http()).await?;

        list.remove(&self.channel_id)
            .ok_or_else(|| anyhow!("invalid channel identifier"))
    }
    pub async fn to_private_channel(self, cache_http: &impl CacheHttp) -> Result<PrivateChannel> {
        self.channel_id
            .to_channel(cache_http)
            .await
            .map_err(Into::into)
            .and_then(|c| c.private().ok_or_else(|| anyhow!("invalid channel type")))
    }
    pub async fn to_message(self, cache_http: &impl CacheHttp) -> Result<Message> {
        if self.guild_id.is_some() {
            self.to_guild_channel(cache_http)
                .await?
                .message(cache_http, self.message_id)
                .await
        } else {
            self.to_private_channel(cache_http)
                .await?
                .message(cache_http, self.message_id)
                .await
        }
        .map_err(Into::into)
    }
}

impl<T: AsRef<Message>> From<T> for Anchor {
    fn from(value: T) -> Self {
        let message = value.as_ref();

        Self {
            guild_id: message.guild_id,
            channel_id: message.channel_id,
            message_id: message.id,
        }
    }
}
