use rocket::{self};
use rocket_contrib::json::Json;
use std::fs::File;
use std::io::Read;
mod models;

fn read_file(filename: String) -> String {
    match File::open(filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Unable to read the file");
            return contents;
        }
        Err(_error) => return String::from("{ \"error\": \"Error opening file\" }"),
    }
}

#[get("/<market_day>/branches.json")]
fn get_market_day_branches(market_day: String) -> Json<Option<Vec<models::Branche>>> {
    let market_day_branches: String = read_file(format!("/tmp/fixxx-pakjekraam/config/markt/{}/branches.json", market_day));
    Json(match serde_json::from_str(&market_day_branches) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            None
        }
    })
}

#[get("/<market_day>/geografie.json")]
fn get_market_day_geography(market_day: String) -> Json<Option<models::Geography>> {
    let market_day_geography: String = read_file(format!("/tmp/fixxx-pakjekraam/config/markt/{}/geografie.json", market_day));
    Json(match serde_json::from_str(&market_day_geography) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            None
        }
    })
}

#[get("/<market_day>/locaties.json")]
fn get_market_day_locations(market_day: String) -> Json<Option<Vec<models::Location>>> {
    let market_day_locations: String = read_file(format!("/tmp/fixxx-pakjekraam/config/markt/{}/locaties.json", market_day));
    Json(match serde_json::from_str(&market_day_locations) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            None
        }
    })
}

#[get("/<market_day>/markt.json")]
fn get_market_day_market(market_day: String) -> Json<Option<models::Market>> {
    let market_day_market: String = read_file(format!("/tmp/fixxx-pakjekraam/config/markt/{}/markt.json", market_day));
    Json(match serde_json::from_str(&market_day_market) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            None
        }
    })
}

#[get("/<market_day>/paginas.json")]
fn get_market_day_pages(market_day: String) -> Json<Option<Vec<models::Page>>> {
    let market_day_pages: String = read_file(format!("/tmp/fixxx-pakjekraam/config/markt/{}/paginas.json", market_day));
    Json(match serde_json::from_str(&market_day_pages) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            None
        }
    })
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount(
        "/api/v1",
        routes![
            get_market_day_branches,
            get_market_day_geography,
            get_market_day_locations,
            get_market_day_market,
            get_market_day_pages,
        ],
    )
}
