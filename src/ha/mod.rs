pub mod dtos;
pub mod error;
mod rest;
mod routes;

use chrono::{DateTime, Utc};
use error::HaError;
use rest::HaRestClient;

use crate::config::SecretIdentifier;
use crate::ha::dtos::{
    HaDomainServiceResponse, HaDomainServiceReturnResponse, HaEventData, HaState,
    HaStateUpdateRequest, HaStateUpdateResponse, ServiceData,
};
use crate::{
    config::ConfigProvider,
    ha::dtos::{
        HaComponentsResponse, HaConfigResponse, HaEventsResponse, HaHistoryOptions,
        HaHistoryResponse, HaLogbookOptions, HaLogbookResponse, HaMessageResponse,
        HaServicesResponse, HaStatesResponse,
    },
};

pub struct HaClient {
    rest: HaRestClient,
}

impl HaClient {
    pub fn new(base_url: String, token: String) -> Self {
        Self {
            rest: HaRestClient::new(base_url, token),
        }
    }

    pub async fn from_config(config_provider: &impl ConfigProvider) -> Result<Self, HaError> {
        let config = config_provider.load_config()?;
        let token = config_provider
            .load_secret(SecretIdentifier::BearerToken)
            .await?;

        Ok(Self::new(config.internal_url, token.value))
    }

    pub async fn api_status(&self) -> Result<HaMessageResponse, HaError> {
        self.rest.api_status().await
    }

    pub async fn get_config(&self) -> Result<HaConfigResponse, HaError> {
        self.rest.get_config().await
    }

    pub async fn get_components(&self) -> Result<HaComponentsResponse, HaError> {
        self.rest.get_components().await
    }

    pub async fn get_events(&self) -> Result<HaEventsResponse, HaError> {
        self.rest.get_events().await
    }

    pub async fn get_services(&self) -> Result<HaServicesResponse, HaError> {
        self.rest.get_services().await
    }

    pub async fn get_history(
        &self,
        from: Option<DateTime<Utc>>,
        query: Option<&HaHistoryOptions>,
        for_entities: Vec<String>,
    ) -> Result<HaHistoryResponse, HaError> {
        self.rest.get_history(from, query, for_entities).await
    }

    pub async fn get_histroy_from_to_timestamp(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        for_entities: Vec<String>,
    ) -> Result<HaHistoryResponse, HaError> {
        let options = HaHistoryOptions {
            end_time: Some(to),
            minimal_response: false,
            no_attributes: false,
            significant_changes_only: false,
        };
        self.rest
            .get_history(Some(from), Some(&options), for_entities)
            .await
    }

    pub async fn get_logbook(
        &self,
        from: Option<DateTime<Utc>>,
        query: Option<&HaLogbookOptions>,
    ) -> Result<HaLogbookResponse, HaError> {
        self.rest.get_logbook(from, query).await
    }

    pub async fn get_logbook_from_to_timestamp(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<HaLogbookResponse, HaError> {
        let options = HaLogbookOptions {
            entity: None,
            end_time: Some(to),
        };
        self.rest.get_logbook(Some(from), Some(&options)).await
    }

    pub async fn get_entity_logbook(
        &self,
        entity_id: String,
    ) -> Result<HaLogbookResponse, HaError> {
        let options = HaLogbookOptions {
            entity: Some(entity_id),
            end_time: None,
        };
        self.rest.get_logbook(None, Some(&options)).await
    }

    pub async fn get_states(&self) -> Result<HaStatesResponse, HaError> {
        self.rest.get_states().await
    }

    pub async fn get_entity_state(&self, entity_id: String) -> Result<HaState, HaError> {
        self.rest.get_entity_state(entity_id).await
    }

    pub async fn get_error_log(&self) -> Result<String, HaError> {
        self.rest.get_error_log().await
    }

    pub async fn update_or_create_state(
        &self,
        state: HaStateUpdateRequest,
        entity_id: String,
    ) -> Result<HaStateUpdateResponse, HaError> {
        self.rest.update_or_create_state(state, entity_id).await
    }

    pub async fn send_event(
        &self,
        event_type: String,
        event_data: Option<HaEventData>,
    ) -> Result<HaMessageResponse, HaError> {
        self.rest.send_event(event_type, event_data).await
    }

    pub async fn call_domain_service(
        &self,
        domain: String,
        service: String,
        service_data: Option<ServiceData>,
    ) -> Result<HaDomainServiceResponse, HaError> {
        self.rest
            .call_domain_service(domain, service, service_data)
            .await
    }

    pub async fn call_domain_service_with_service_response(
        &self,
        domain: String,
        service: String,
        service_data: Option<ServiceData>,
    ) -> Result<HaDomainServiceReturnResponse, HaError> {
        self.rest
            .call_domain_service_with_service_response(domain, service, service_data)
            .await
    }
}
