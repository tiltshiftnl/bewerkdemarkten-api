use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Announcement {
    #[serde(rename(serialize = "activatie", deserialize = "activatie"))]
    activation: String,
    #[serde(rename(serialize = "wenperiode", deserialize = "wenperiode"))]
    staging_period: String,
    live: String,
}

#[derive(Serialize, Deserialize)]
pub struct Announcements {
    #[serde(rename(serialize = "marktDetail", deserialize = "marktDetail"))]
    market_detail: Announcement,
    #[serde(rename(
        serialize = "marktDetailPlaatsvoorkeuren",
        deserialize = "marktDetailPlaatsvoorkeuren"
    ))]
    market_detail_properties: Announcement,
    #[serde(rename(serialize = "aanwezigheid", deserialize = "aanwezigheid"))]
    presence: Announcement,
    #[serde(rename(serialize = "plaatsVoorkeuren", deserialize = "plaatsVoorkeuren"))]
    properties: Announcement,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Branche {
    #[serde(alias = "brancheId")]
    branche_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<i32>,
    description: Option<String>,
    color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    weekday: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plan: Option<Plan>,
}

impl Event {
    pub fn new(weekday: Option<i32>, plan: Option<Plan>) -> Self {
        Event {
            weekday: weekday,
            plan: plan,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Market {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plan: Option<Plan>,
    pub events: HashMap<String, Event>,
}

impl Market {
    pub fn new(id: i32, name: Option<String>) -> Self {
        Market {
            id: Some(id),
            name: name,
            events: HashMap::new(),
            plan: None,
            phase: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Plan {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pages: Option<i32>,
}

impl Plan {
    pub fn new(name: String) -> Self {
        Plan {
            name: name,
            pages: None,
        }
    }
}