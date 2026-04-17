use std::fmt::format;

use crate::ha::dtos::{
    Components, DomainServiceResponse, DomainServiceReturnResponse, EventData, Events, HaConfig,
    HaMessage, HistoryOptions, HistoryQuery, HistoryResponse, LogbookOptions, LogbookResponse,
    ServiceData, Services, StateObject, StateUpdateRequest, StateUpdateResponse, StatesResponse,
};
use crate::ha::error::HaError;
use crate::ha::routes::{
    COMPONENTS, CONFIG, ERROR_LOG, EVENTS, HISTORY, LOGBOOK, SERVICES, STATES,
};
use chrono::{DateTime, Utc};
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
    pub async fn api_status(&self) -> Result<HaMessage, HaError> {
        self.get::<(), HaMessage>("", None).await
    }

    pub async fn get_config(&self) -> Result<HaConfig, HaError> {
        self.get::<(), HaConfig>(CONFIG, None).await
    }

    pub async fn get_components(&self) -> Result<Components, HaError> {
        self.get::<(), Components>(COMPONENTS, None).await
    }

    pub async fn get_events(&self) -> Result<Events, HaError> {
        self.get::<(), Events>(EVENTS, None).await
    }

    pub async fn get_services(&self) -> Result<Services, HaError> {
        self.get::<(), Services>(SERVICES, None).await
    }

    pub async fn get_history(
        &self,
        timestamp: Option<DateTime<Utc>>,
        query: Option<&HistoryOptions>,
        entity_ids: Vec<String>,
    ) -> Result<HistoryResponse, HaError> {
        if entity_ids.is_empty() {
            return Err(HaError::MissingEntityId);
        }

        let query_params = if let Some(query) = query {
            HistoryQuery::from_query_options(query, entity_ids)
        } else {
            HistoryQuery::from_default(entity_ids)
        };

        let mut path = HISTORY.to_owned();
        if let Some(timestamp) = timestamp {
            path = format!("{path}/{}", timestamp);
        }

        self.get::<HistoryQuery, HistoryResponse>(&path, Some(&query_params))
            .await
    }

    pub async fn get_logbook(
        &self,
        timestamp: Option<DateTime<Utc>>,
        query: Option<&LogbookOptions>,
    ) -> Result<LogbookResponse, HaError> {
        let mut path = LOGBOOK.to_owned();
        if let Some(timestamp) = timestamp {
            path = format!("{path}/{}", timestamp);
        }

        self.get::<LogbookOptions, LogbookResponse>(&path, query)
            .await
    }

    pub async fn get_states(&self) -> Result<StatesResponse, HaError> {
        self.get::<(), StatesResponse>(STATES, None).await
    }

    pub async fn get_entity_state(&self, entity_id: String) -> Result<StateObject, HaError> {
        let path = format!("{}/{}", STATES, entity_id);
        self.get::<(), StateObject>(&path, None).await
    }

    pub async fn get_error_log(&self) -> Result<String, HaError> {
        self.get::<(), String>(ERROR_LOG, None).await
    }

    pub async fn update_or_create_state(
        &self,
        state: StateUpdateRequest,
        entity_id: String,
    ) -> Result<StateUpdateResponse, HaError> {
        let path = format!("{STATES}/{entity_id}");
        self.post(&path, state, None::<&()>).await // TURBO c><(
    }

    pub async fn send_event(
        &self,
        event_type: String,
        event_data: Option<EventData>,
    ) -> Result<HaMessage, HaError> {
        let path = format!("{}/{}", EVENTS, event_type);
        self.post(&path, event_data, None::<&()>).await
    }

    pub async fn call_domain_service(
        &self,
        domain: String,
        service: String,
        service_data: Option<ServiceData>,
    ) -> Result<DomainServiceResponse, HaError> {
        let path = format!("{SERVICES}/{}/{}", domain, service);
        self.post(&path, service_data, None::<&()>).await
    }

    pub async fn call_domain_service_with_service_response(
        &self,
        domain: String,
        service: String,
        service_data: Option<ServiceData>,
    ) -> Result<DomainServiceReturnResponse, HaError> {
        let path = format!("{SERVICES}/{}/{}", domain, service);
        let query_param = ("return_response", true);

        self.post(&path, service_data, Some(&query_param)).await
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

    async fn get<Q: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        query: Option<&Q>,
    ) -> Result<T, HaError> {
        let mut request = self.client.get(self.url(path)).bearer_auth(&self.token);

        if let Some(query) = query {
            request = request.query(query);
        }

        let response = request.send().await?;

        Self::handle_response(response).await
    }

    async fn post<Q: Serialize, B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: B,
        query: Option<&Q>,
    ) -> Result<T, HaError> {
        let mut request = self
            .client
            .post(self.url(path))
            .bearer_auth(&self.token)
            .json(&body);

        if let Some(query) = query {
            request = request.query(query);
        }

        let response = request.send().await?;

        Self::handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(
        response: reqwest::Response,
    ) -> Result<T, HaError> {
        let status_code = response.status();
        let text = response.text().await?;

        if status_code == StatusCode::UNAUTHORIZED {
            return Err(HaError::Unauthorized);
        }

        if !status_code.is_success() {
            return Err(HaError::Http {
                status: status_code,
                body: text,
            });
        }

        from_str::<T>(&text).map_err(|e| HaError::Decode {
            source: e,
            body: text,
        })
    }
}
