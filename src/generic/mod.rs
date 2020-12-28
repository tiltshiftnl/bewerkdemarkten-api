use rocket::{self};
use rocket_contrib::json::Json;
use std::fs::File;
use std::io::Read;
mod models;

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
        ],
    )
}
