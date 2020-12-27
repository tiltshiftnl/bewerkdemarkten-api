use serde::{Deserialize, Serialize};

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
    #[serde(rename(serialize = "marktDetailPlaatsvoorkeuren", deserialize = "marktDetailPlaatsvoorkeuren"))]
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