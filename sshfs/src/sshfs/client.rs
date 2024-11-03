use async_trait::async_trait;
use russh::{client, keys::key, ChannelId};
use tracing::info;

pub struct Sshfs {}

impl Sshfs {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl client::Handler for Sshfs {
    type Error = anyhow::Error;
    async fn check_server_key(
        &mut self,
        server_public_key: &key::PublicKey,
    ) -> Result<bool, Self::Error> {
        info!("check_server_key: {:?}", server_public_key);
        Ok(true)
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        _session: &mut client::Session,
    ) -> Result<(), Self::Error> {
        info!("data on channel {:?}: {}", channel, data.len());
        Ok(())
    }
}
