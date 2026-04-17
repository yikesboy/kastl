use crate::ha::dtos::{
    HaComponentsResponse, HaConfigResponse, HaDomainServiceResponse, HaDomainServiceReturnResponse,
    HaEventData, HaEventsResponse, HaHistoryOptions, HaHistoryQuery, HaHistoryResponse,
    HaLogbookOptions, HaLogbookResponse, HaMessageResponse, HaServicesResponse, HaState,
    HaStateUpdateRequest, HaStateUpdateResponse, HaStatesResponse, ServiceData,
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
    pub async fn api_status(&self) -> Result<HaMessageResponse, HaError> {
        self.get::<(), HaMessageResponse>("", None).await
    }

    pub async fn get_config(&self) -> Result<HaConfigResponse, HaError> {
        self.get::<(), HaConfigResponse>(CONFIG, None).await
    }

    pub async fn get_components(&self) -> Result<HaComponentsResponse, HaError> {
        self.get::<(), HaComponentsResponse>(COMPONENTS, None).await
    }

    pub async fn get_events(&self) -> Result<HaEventsResponse, HaError> {
        self.get::<(), HaEventsResponse>(EVENTS, None).await
    }

    pub async fn get_services(&self) -> Result<HaServicesResponse, HaError> {
        self.get::<(), HaServicesResponse>(SERVICES, None).await
    }

    pub async fn get_history(
        &self,
        timestamp: Option<DateTime<Utc>>,
        query: Option<&HaHistoryOptions>,
        entity_ids: Vec<String>,
    ) -> Result<HaHistoryResponse, HaError> {
        if entity_ids.is_empty() {
            return Err(HaError::MissingEntityId);
        }

        let query_params = if let Some(query) = query {
            HaHistoryQuery::from_query_options(query, entity_ids)
        } else {
            HaHistoryQuery::from_default(entity_ids)
        };

        let mut path = HISTORY.to_owned();
        if let Some(timestamp) = timestamp {
            path = format!("{path}/{}", timestamp);
        }

        self.get::<HaHistoryQuery, HaHistoryResponse>(&path, Some(&query_params))
            .await
    }

    pub async fn get_logbook(
        &self,
        timestamp: Option<DateTime<Utc>>,
        query: Option<&HaLogbookOptions>,
    ) -> Result<HaLogbookResponse, HaError> {
        let mut path = LOGBOOK.to_owned();
        if let Some(timestamp) = timestamp {
            path = format!("{path}/{}", timestamp);
        }

        self.get::<HaLogbookOptions, HaLogbookResponse>(&path, query)
            .await
    }

    pub async fn get_states(&self) -> Result<HaStatesResponse, HaError> {
        self.get::<(), HaStatesResponse>(STATES, None).await
    }

    pub async fn get_entity_state(&self, entity_id: String) -> Result<HaState, HaError> {
        let path = format!("{}/{}", STATES, entity_id);
        self.get::<(), HaState>(&path, None).await
    }

    pub async fn get_error_log(&self) -> Result<String, HaError> {
        self.get::<(), String>(ERROR_LOG, None).await
    }

    pub async fn update_or_create_state(
        &self,
        state: HaStateUpdateRequest,
        entity_id: String,
    ) -> Result<HaStateUpdateResponse, HaError> {
        let path = format!("{STATES}/{entity_id}");
        self.post(&path, state, None::<&()>).await // TURBO c><(
    }

    pub async fn send_event(
        &self,
        event_type: String,
        event_data: Option<HaEventData>,
    ) -> Result<HaMessageResponse, HaError> {
        let path = format!("{}/{}", EVENTS, event_type);
        self.post(&path, event_data, None::<&()>).await
    }

    pub async fn call_domain_service(
        &self,
        domain: String,
        service: String,
        service_data: Option<ServiceData>,
    ) -> Result<HaDomainServiceResponse, HaError> {
        let path = format!("{SERVICES}/{}/{}", domain, service);
        self.post(&path, service_data, None::<&()>).await
    }

    pub async fn call_domain_service_with_service_response(
        &self,
        domain: String,
        service: String,
        service_data: Option<ServiceData>,
    ) -> Result<HaDomainServiceReturnResponse, HaError> {
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
