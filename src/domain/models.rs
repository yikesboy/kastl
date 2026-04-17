use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

use crate::ha::dtos::{
    self, HaComponentsResponse, HaConfigResponse, HaEventData, HaEventsResponse, HaMessageResponse,
};

pub struct Message {
    pub message: String,
}

impl From<dtos::HaMessageResponse> for Message {
    fn from(item: dtos::HaMessageResponse) -> Self {
        Message {
            message: item.message,
        }
    }
}

pub struct Config {
    pub components: Vec<String>,
    pub config_dir: String,
    pub elevation: i64,
    pub latitude: f64,
    pub location_name: String,
    pub longitude: f64,
    pub time_zone: String,
    pub unit_system: UnitSystem,
    pub version: String,
    pub whitelist_external_dirs: Vec<String>,
}

impl From<dtos::HaConfigResponse> for Config {
    fn from(item: dtos::HaConfigResponse) -> Self {
        let unit_system = UnitSystem::from(item.unit_system);
        Config {
            components: item.components,
            config_dir: item.config_dir,
            elevation: item.elevation,
            latitude: item.latitude,
            location_name: item.location_name,
            longitude: item.longitude,
            time_zone: item.time_zone,
            unit_system: unit_system,
            version: item.version,
            whitelist_external_dirs: item.whitelist_external_dirs,
        }
    }
}

pub struct UnitSystem {
    pub length: String,
    pub mass: String,
    pub temperature: String,
    pub volume: String,
}

impl From<dtos::HaUnitSystem> for UnitSystem {
    fn from(item: dtos::HaUnitSystem) -> Self {
        UnitSystem {
            length: item.length,
            mass: item.mass,
            temperature: item.temperature,
            volume: item.volume,
        }
    }
}

pub struct Components(pub Vec<Component>);
pub type Component = String;

impl From<dtos::HaComponentsResponse> for Components {
    fn from(item: dtos::HaComponentsResponse) -> Self {
        Components(item.0)
    }
}

pub struct Events(pub Vec<Event>);

impl From<dtos::HaEventsResponse> for Events {
    fn from(item: dtos::HaEventsResponse) -> Self {
        let events = item.0.into_iter().map(Event::from).collect();
        Events(events)
    }
}

pub struct Event {
    pub event: String,
    pub listener_count: i32,
}

impl From<dtos::HaEvent> for Event {
    fn from(item: dtos::HaEvent) -> Self {
        Event {
            event: item.event,
            listener_count: item.listener_count,
        }
    }
}

pub struct Services(pub Vec<Service>);

impl From<dtos::HaServicesResponse> for Services {
    fn from(item: dtos::HaServicesResponse) -> Self {
        let services = item.0.into_iter().map(Service::from).collect();
        Services(services)
    }
}

pub struct Service {
    pub domain: String,
    pub services: Vec<String>,
}

impl From<dtos::HaService> for Service {
    fn from(item: dtos::HaService) -> Self {
        Service {
            domain: item.domain,
            services: item.services,
        }
    }
}

pub struct History(pub Vec<Vec<StateChange>>);

impl From<dtos::HaHistoryResponse> for History {
    fn from(item: dtos::HaHistoryResponse) -> Self {
        let history: Vec<Vec<StateChange>> = item
            .0
            .into_iter()
            .map(|scv| scv.into_iter().map(StateChange::from).collect())
            .collect();
        History(history)
    }
}

pub struct StateChange {
    pub attributes: Option<StateChangeAttribute>,
    pub entity_id: Option<String>,
    pub last_changed: DateTime<Utc>,
    pub last_updated: Option<DateTime<Utc>>,
    pub state: String,
}

impl From<dtos::HaStateChange> for StateChange {
    fn from(item: dtos::HaStateChange) -> Self {
        let attributes = item.attributes.map(StateChangeAttribute::from);
        StateChange {
            attributes: attributes,
            entity_id: item.entity_id,
            last_changed: item.last_changed,
            last_updated: item.last_updated,
            state: item.state,
        }
    }
}

pub struct StateChangeAttribute {
    pub friendly_name: Option<String>,
    pub unit_of_measurement: Option<String>,
}

impl From<dtos::HaStateChangeAttribute> for StateChangeAttribute {
    fn from(item: dtos::HaStateChangeAttribute) -> Self {
        StateChangeAttribute {
            friendly_name: item.friendly_name,
            unit_of_measurement: item.unit_of_measurement,
        }
    }
}

pub struct Logbook(pub Vec<LogbookEntry>);

impl From<dtos::HaLogbookResponse> for Logbook {
    fn from(item: dtos::HaLogbookResponse) -> Self {
        let logbooks = item.0.into_iter().map(LogbookEntry::from).collect();
        Logbook(logbooks)
    }
}

pub struct LogbookEntry {
    pub context_user_id: Option<String>,
    pub domain: String,
    pub entity_id: String,
    pub message: String,
    pub name: String,
    pub when: DateTime<Utc>,
}

impl From<dtos::HaLogbookEntry> for LogbookEntry {
    fn from(item: dtos::HaLogbookEntry) -> Self {
        LogbookEntry {
            context_user_id: item.context_user_id,
            domain: item.domain,
            entity_id: item.entity_id,
            message: item.message,
            name: item.name,
            when: item.when,
        }
    }
}

pub struct States(pub Vec<State>);

impl From<dtos::HaStatesResponse> for States {
    fn from(item: dtos::HaStatesResponse) -> Self {
        let states = item.0.into_iter().map(State::from).collect();
        States(states)
    }
}

pub struct State {
    pub attributes: HashMap<String, Value>,
    pub entity_id: String,
    pub last_changed: DateTime<Utc>,
    pub state: String,
}

impl From<dtos::HaState> for State {
    fn from(item: dtos::HaState) -> Self {
        State {
            attributes: item.attributes,
            entity_id: item.entity_id,
            last_changed: item.last_changed,
            state: item.state,
        }
    }
}

pub struct StateUpdate {
    pub attributes: HashMap<String, Value>,
    pub entity_id: String,
    pub last_changed: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub state: String,
}

impl From<dtos::HaStateUpdateResponse> for StateUpdate {
    fn from(item: dtos::HaStateUpdateResponse) -> Self {
        StateUpdate {
            attributes: item.attributes,
            entity_id: item.entity_id,
            last_changed: item.last_changed,
            last_updated: item.last_updated,
            state: item.state,
        }
    }
}

pub type EventData = dtos::HaEventData;

pub struct DomainService(pub Vec<State>);

impl From<dtos::HaDomainServiceResponse> for DomainService {
    fn from(item: dtos::HaDomainServiceResponse) -> Self {
        let domain_services = item.0.into_iter().map(State::from).collect();
        DomainService(domain_services)
    }
}

pub struct DomainServiceReturn {
    pub changed_states: Vec<State>,
    pub service_response: HashMap<String, Value>,
}

impl From<dtos::HaDomainServiceReturnResponse> for DomainServiceReturn {
    fn from(item: dtos::HaDomainServiceReturnResponse) -> Self {
        let changed_states = item.changed_states.into_iter().map(State::from).collect();
        DomainServiceReturn {
            changed_states: changed_states,
            service_response: item.service_response,
        }
    }
}
