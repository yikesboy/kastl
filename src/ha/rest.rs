use crate::ha::error::HaError;
use crate::ha::model::{HaConfig, HaStatusMessage};
use crate::ha::routes::{CONFIG};
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::from_str;

pub struct HaRestClient {
    client: Client,
    base_url: String,
    token: String,
}

impl HaRestClient {
    pub async fn api_status(&self) -> Result<HaStatusMessage, HaError> {
        self.get::<HaStatusMessage>("").await
    }

    pub async fn get_config(&self) -> Result<HaConfig, HaError> {
        self.get::<HaConfig>(CONFIG).await
    }
}

impl HaRestClient {
    pub fn new(base_url: String, token: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            token,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}/api/{}", self.base_url, path)
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, HaError> {
        let response = self.client
            .get(self.url(path))
            .bearer_auth(&self.token)
            .send()
            .await?;

        Self::handle_response(response).await
    }

    async fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: B,
    ) -> Result<T, HaError> {
        let response = self.client
            .post(self.url(path))
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await?;

        Self::handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(response: reqwest::Response) -> Result<T, HaError> {
        let status_code = response.status();
        let text = response.text().await?;

        if status_code == StatusCode::UNAUTHORIZED {
            return Err(HaError::Unauthorized);
        }

        if !status_code.is_success() {
            return Err(HaError::Http { status: status_code, body: text });
        }

        from_str::<T>(&text).map_err(|e| HaError::Decode {
            source: e,
            body: text,
        })
    }
}
