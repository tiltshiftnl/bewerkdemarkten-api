use rocket::{self};
use rocket_contrib::json::Json;
use std::io::{Read, Result};
use std::fs::{self, File};
use std::path::Path;
use std::collections::HashMap;
mod models;

fn visit_dirs() -> Result<HashMap<String, models::Market>> {
    let dir = Path::new("/tmp/fixxx-pakjekraam/config/markt");
    let mut markets: HashMap<String, models::Market> = HashMap::new();
    if dir.is_dir() {
        let mut i: i32 = 0;
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                match entry.file_name().into_string() {
                    Ok(file_name) => {
                        
                        let market: models::Market = models::Market::new(
                            i,
                            file_name.rsplit("-").take(1).collect(),
                            "".to_string(),
                        );
                        markets.insert(
                            file_name.split("-").take(1).collect(),
                            market
                        );
                        i = i + 1;
                    },
                    Err(_) => { /* Do nothing */},
                }
            }
        }
    }
    Ok(markets)
}


fn read_file(filename: &str) -> String {
    match File::open(filename.to_string()) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Unable to read the file");
            return contents;
        }
        Err(_error) => return "{ \"error\": \"Error opening file\" }".to_string(),
    }
}

#[get("/mededelingen.json")]
fn get_announcements() -> Json<Option<models::Announcements>> {
    let announcements: String = read_file("/tmp/fixxx-pakjekraam/config/markt/mededelingen.json");
    Json(match serde_json::from_str(&announcements) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            None
        }
    })
}

#[get("/branches.json")]
fn get_branches() -> Json<Vec<models::Branche>> {
    let branches: String = read_file("/tmp/fixxx-pakjekraam/config/markt/branches.json");
    Json(match serde_json::from_str(&branches) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            vec![]
        }
    })
}

#[get("/daysClosed.json")]
fn get_days_closed() -> Json<Vec<String>> {
    let days_closed: String = read_file("/tmp/fixxx-pakjekraam/config/markt/daysClosed.json");

    Json(match serde_json::from_str(&days_closed) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            vec!["error".to_string()]
        }
    })
}

#[get("/markets.json")]
fn get_markets() -> Json<Option<HashMap<String, models::Market>>> {
    Json(match visit_dirs() {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Fail: {}", e);
            None
        },
    })
}

#[get("/obstakeltypes.json")]
fn get_obstacle_types() -> Json<Vec<String>> {
    let obstacle_types: String = read_file("/tmp/fixxx-pakjekraam/config/markt/obstakeltypes.json");
    Json(match serde_json::from_str(&obstacle_types) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            vec!["error".to_string()]
        }
    })
}

#[get("/plaatseigenschappen.json")]
fn get_properties() -> Json<Vec<String>> {
    let properties: String =
        read_file("/tmp/fixxx-pakjekraam/config/markt/plaatseigenschappen.json");
    Json(match serde_json::from_str(&properties) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            vec!["error".to_string()]
        }
    })
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount(
        "/api/v1",
        routes![
            get_days_closed,
            get_obstacle_types,
            get_properties,
            get_branches,
            get_announcements,
            get_markets,
        ],
    )
}
