use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Branche {
    #[serde(rename(serialize = "brancheId", deserialize = "brancheId"))]
    branche_id: String,
    #[serde(
        rename(serialize = "verplicht", deserialize = "verplicht"),
        skip_serializing_if = "Option::is_none"
    )]
    mandatory: Option<bool>,
    #[serde(
        rename(serialize = "maximumPlaatsen", deserialize = "maximumPlaatsen"),
        skip_serializing_if = "Option::is_none"
    )]
    maximum: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Obstacle {
    #[serde(rename(serialize = "kraamA", deserialize = "kraamA"))]
    from_location: String,
    #[serde(rename(serialize = "kraamB", deserialize = "kraamB"))]
    to_location: String,
    #[serde(rename(serialize = "obstakel", deserialize = "obstakel"))]
    obstacle: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Geography {
    #[serde(rename(serialize = "obstakels", deserialize = "obstakels"))]
    obstacles: Vec<Obstacle>,
}

impl Geography {
    pub fn new() -> Self {
        Geography {
            obstacles: vec![],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    #[serde(rename(serialize = "plaatsId", deserialize = "plaatsId"))]
    place_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<Vec<String>>,
    #[serde(rename(serialize = "verkoopinrichting", deserialize = "verkoopinrichting"),
    skip_serializing_if = "Option::is_none")]
    options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Rows {
    rows: Vec<Option<Vec<String>>>,
}

impl Rows {
    pub fn new() -> Self {
        Rows {
            rows: vec![],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ListGroup {
    class: String,
    title: String,
    #[serde(rename(serialize = "landmarkTop", deserialize = "landmarkTop"))]
    landmark_top: String,
    #[serde(rename(serialize = "landmarkBottom", deserialize = "landmarkBottom"))]
    landmark_bottom: String,
    #[serde(rename(serialize = "plaatsList", deserialize = "plaatsList"))]
    place_list: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Page {
    title: String,
    #[serde(rename(serialize = "indelingslijstGroup", deserialize = "indelingslijstGroup"))]
    list_group: Vec<ListGroup>,
}
