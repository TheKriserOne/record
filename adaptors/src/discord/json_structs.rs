use crate::types::{Conversation, User as GlobalUser, Guild as GlobalGuild};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
// === Users ===

#[derive(Deserialize)]
pub struct Profile {
    accent_color: Option<String>,
    authenticator_types: Vec<String>,
    avatar: Option<String>,
    avatar_decoration_data: Option<String>,
    banner: Option<String>,
    banner_color: Option<String>,
    bio: String,
    clan: Option<String>,
    discriminator: String,
    email: String,
    flags: i32,
    global_name: String,
    id: String,
    linked_users: Vec<String>,
    locale: String,
    mfa_enabled: bool,
    nsfw_allowed: bool,
    phone: Option<String>,
    premium_type: i32,
    public_flags: i32,
    username: String,
    verified: bool,
}
impl Into<GlobalUser> for Profile {
    fn into(self) -> GlobalUser {
        GlobalUser {
            id: self.id,
            username: self.username,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct User {
    avatar: Option<String>,
    avatar_decoration_data: Option<String>,
    clan: Option<String>,
    discriminator: String,
    id: String,
    username: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Friend {
    id: String,
    is_spam_request: bool,
    nickname: Option<String>,
    since: String,
    // type: i32,
    user: User,
}
impl Into<GlobalUser> for Friend {
    fn into(self) -> GlobalUser {
        GlobalUser {
            id: self.id,
            username: self.user.username,
        }
    }
}
#[derive(Deserialize, Debug, Clone)]
pub struct Recipient {
    pub(crate) avatar: Option<String>,
    avatar_decoration_data: Option<String>,
    clan: Option<String>,
    discriminator: String,
    global_name: Option<String>,
    pub(crate) id: String,
    public_flags: i32,
    username: String,
}

// === Chennels ===

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum ChannelTypes {
    GuildText,
    DM,
    GuildVoice,
    GroupDM,
    GuildCategory,
    GuildAnnouncement,
    AnnouncementThread,
    PublicThread,
    PrivateThread,
    GuildStageVoice,
    GuildDirectory,
    GuildForum,
    GuildMedia,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Channel {
    pub(crate) id: String,
    #[serde(rename = "type")]
    channel_type: ChannelTypes,
    flags: i32,
    pub(crate) icon: Option<String>,
    last_message_id: Option<String>,
    name: Option<String>,
    pub(crate) recipients: Vec<Recipient>,
}
impl Into<Conversation> for Channel {
    fn into(self) -> Conversation {
        Conversation {
            id: self.id,
            name: self.name.unwrap_or(match self.recipients.get(0) {
                Some(test) => test.username.clone(),
                None => "Fix later".to_string(),
            }),
            icon: match self.channel_type {
                ChannelTypes::DM => self.recipients.get(0).and_then(|r| r.avatar.clone()),
                ChannelTypes::GroupDM => self.icon,
                _ => None,
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CountDetails {
    burst: u32,
    normal: u32,
}

#[derive(Deserialize, Debug)]
pub struct Emoji {
    id: Option<String>,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Reaction {
    burst_colors: Vec<String>,
    burst_count: u32,
    burst_me: bool,
    count: u32,
    count_details: CountDetails,
    emoji: Emoji,
    me: bool,
    me_burst: bool,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    attachments: Vec<String>,
    author: User,
    channel_id: String,
    components: Vec<String>,
    content: String,
    edited_timestamp: Option<String>,
    embeds: Vec<u32>,
    flags: u32,
    id: String,
    mention_everyone: bool,
    // mention_roles: Vec<String>,
    // mentions: Vec<String>,
    pinned: bool,
    reactions: Option<Vec<Reaction>>,
    timestamp: String,
    tts: bool,
    // type: u32,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    // pub icon_hash: Option<String>,
    // pub splash: Option<String>,
    // pub discovery_splash: Option<String>,
    // pub owner: Option<bool>,
    // pub owner_id: String,  // Snowflake
    // pub permissions: Option<String>,
    // pub region: Option<String>,  // Deprecated
    // pub afk_channel_id: Option<String>,  // Snowflake
    // pub afk_timeout: u32,
    // pub widget_enabled: Option<bool>,
    // pub widget_channel_id: Option<String>,  // Snowflake
    // pub verification_level: u8,
    // pub default_message_notifications: u8,
    // pub explicit_content_filter: u8,
    // pub roles: Vec<Role>,
    // pub emojis: Vec<Emoji>,
    // pub features: Vec<String>,
    // pub mfa_level: u8,
    // pub application_id: Option<String>,  // Snowflake
    // pub system_channel_id: Option<String>,  // Snowflake
    // pub system_channel_flags: u32,
    // pub rules_channel_id: Option<String>,  // Snowflake
    // pub max_presences: Option<u32>,
    // pub max_members: Option<u32>,
    // pub vanity_url_code: Option<String>,
    // pub description: Option<String>,
    // pub banner: Option<String>,
    // pub premium_tier: u8,
    // pub premium_subscription_count: Option<u32>,
    // pub preferred_locale: String,
    // pub public_updates_channel_id: Option<String>,  // Snowflake
    // pub max_video_channel_users: Option<u32>,
    // pub max_stage_video_channel_users: Option<u32>,
    // pub approximate_member_count: Option<u32>,
    // pub approximate_presence_count: Option<u32>,
    // pub welcome_screen: Option<WelcomeScreen>,
    // pub nsfw_level: u8,
    // pub stickers: Option<Vec<Sticker>>,
    // pub premium_progress_bar_enabled: bool,
    // pub safety_alerts_channel_id: Option<String>,  // Snowflake
    // pub incidents_data: Option<IncidentsData>,
}

impl Into<GlobalGuild> for Guild {
    fn into(self) -> GlobalGuild {
        GlobalGuild {
            id: self.id,
            name: self.name,
            icon: self.icon,
        }
    }
}