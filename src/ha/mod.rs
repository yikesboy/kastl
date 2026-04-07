mod rest;
pub mod error;
mod model;
mod routes;

use rest::HaRestClient;
use error::HaError;

use crate::{config::ConfigProvider, ha::model::{HaConfig, HaStatusMessage}};
use crate::config::SecretIdentifier;

pub struct HaClient {
    rest: HaRestClient
}

impl HaClient {
    pub fn new(base_url: String, token: String) -> Self {
        Self {
            rest: HaRestClient::new(base_url, token),
        }
    }

    pub async fn from_config(config_provider: &impl ConfigProvider) -> Result<Self, HaError> {
        let config = config_provider.load_config()?;
        let token = config_provider.load_secret(SecretIdentifier::BearerToken).await?;

        Ok(Self::new(config.internal_url, token.value))
    }

    pub async fn api_status(&self) -> Result<HaStatusMessage, HaError> {
        self.rest.api_status().await
    }

    pub async fn get_config(&self) -> Result<HaConfig, HaError> {
        self.rest.get_config().await
    }
}
