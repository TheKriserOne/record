use std::error::Error;
use std::future::Future;
use std::path::PathBuf;
use async_trait::async_trait;
use futures::{stream, try_join, StreamExt};
use futures::future::try_join_all;
use crate::{
    network::http_request,
    types::{Conversation, Guild as GlobalGuild, User},
    MessageLocation, MessangerQuery, ParameterizedMessangerQuery,
};
use crate::network::cache_download;
use super::{json_structs::{Channel, Friend, Profile, Guild}, Discord};

impl Discord {
    fn get_auth_header(&self) -> Vec<(&str, String)> {
        vec![("Authorization", self.token.clone())]
    }
}

#[async_trait]
impl MessangerQuery for Discord {
    async fn get_profile(&self) -> Result<User, Box<dyn Error>> {
        let profile = http_request::<Profile>(
            surf::get("https://discord.com/api/v9/users/@me"),
            self.get_auth_header(),
        )
            .await?;
        Ok(profile.into())
    }
    async fn get_contacts(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let friends = http_request::<Vec<Friend>>(
            surf::get("https://discord.com/api/v9/users/@me/relationships"),
            self.get_auth_header(),
        ).await?;
        Ok(friends.iter().map(|friend| friend.clone().into()).collect())
    }
    async fn get_conversation(&self) -> Result<Vec<Conversation>, Box<dyn Error>> {
        let channels = http_request::<Vec<Channel>>(
            surf::get("https://discord.com/api/v10/users/@me/channels"),
            self.get_auth_header(),
        ).await?;
        println!("Channels: {:#?}", channels);
        let futures: Vec<_> = channels
            .iter()
            .map(async move |item| -> Result<(), Box<dyn Error + Send>> {
                if let Some(icon) = &item.icon {
                    let url = format!(
                        "https://cdn.discordapp.com/channel-icons/{}/{}.png",
                        item.id, icon
                    );
                    cache_download(url, PathBuf::from("./cache/discord/channels/imgs/"), format!("{}.png", item.id)).await;
                }
                else if let Some(icon) = &item.recipients[0].avatar {
                    let url = format!(
                        "https://cdn.discordapp.com/avatars/{}/{}.png",
                        item.recipients[0].id, icon
                    );
                    cache_download(url, PathBuf::from("./cache/discord/channels/imgs/"), format!("{}.png", item.id)).await;
                }
                Ok(())
            })
            .collect();
        try_join_all(futures).await.expect("Error downloading channel photos");
        Ok(channels
            .iter()
            .map(|channel| channel.clone().into())
            .collect())
    }
    async fn get_guilds(&self) -> Result<Vec<GlobalGuild>, Box<dyn Error>> {
        let guilds = http_request::<Vec<Guild>>(
            surf::get("https://discord.com/api/v10/users/@me/guilds"),
            self.get_auth_header(),
        ).await?;

        let futures: Vec<_> = guilds
            .iter()
            .map(async move |item| -> Result<(), Box<dyn Error + Send>> {
                if let Some(icon) = &item.icon {
                    let url = format!(
                        "https://cdn.discordapp.com/icons/{}/{}.png",
                        item.id, icon
                    );
                    cache_download(url, PathBuf::from("./cache/discord/guilds/imgs/"), format!("{}.png", item.id)).await;
                }
                Ok(())
            })
            .collect();
        try_join_all(futures).await.expect("Error downloading guild photos");

        Ok(guilds
            .iter()
            .map(|guild| guild.clone().into())
            .collect())
    }
}

#[async_trait]
impl ParameterizedMessangerQuery for Discord {
    async fn get_messanges(&self, before_message: &dyn MessageLocation) -> Result<(), surf::Error> {
        todo!()
    }
}
