use rocket::{self};
use rocket_contrib::json::Json;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Result};
use std::path::{Path, PathBuf};
use bewerkdemarkten_api::models::generic::{Announcements, Branche, Event, Market, Plan};

fn set_market(markets: &HashMap<String, Market>, market_id: &String, day_id: &String, i: &i32) -> Market {
    let weekday: Option<i32> = match &day_id[..] {
        "MA" => Some(1),
        "DI" => Some(2),
        "WO" => Some(3),
        "DO" => Some(4),
        "VR" => Some(5),
        "ZA" => Some(6),
        "ZO" => Some(7),
        _ => None,
    };

    let mut new_market: Market = Market::new(
        *i,
        None,
    );

    if markets.contains_key(market_id) {
        let market = markets.get(market_id).unwrap();
        // Insert cloned events from previous state
        for (key, value) in market.events.iter() {
            new_market.events.insert(key.clone(), value.clone());
        }
    }
    // TODO Check if there is a pdf available for this market day?
    match find_pdf_file(format!("{}-{}-{}.pdf", "kaart", market_id.to_string(), day_id)) {
        Some(filepath) => {
            let name: String = match filepath.file_stem(){
                Some(n) => match n.to_str() {
                    Some(m) => String::from(m.to_string()),
                    None => String::from("")
                },
                None => String::from(""),
            };
            // Add the new event.
            new_market.events.insert(
                day_id.clone(),
                Event::new(
                    weekday,
                    Some(Plan::new(name)),
                ),
            );
        },
        //println!("Pdf found: {:?}", filepath.file_stem()),
        None => {
            println!("No pdf found.");
            new_market.events.insert(
                day_id.clone(),
                Event::new(
                    weekday,
                    None
                ),
            );
        },
    };
    // // Add the new event.
    // new_market.events.insert(
    //     day_id.clone(),
    //     models::Event::new(
    //         weekday,
    //         None
    //     ),
    // );
    new_market
}


/**
 * Find a rusv file in the current or parent directories of the given directory.
 */
fn find_pdf_file(filename: String) -> Option<PathBuf> {
    let mut directory = Path::new("/tmp/fixxx-pakjekraam/dist/pdf");
    let pdf_filename = Path::new(&filename);

    loop {
        let filepath: PathBuf = [
            directory,
            pdf_filename
        ].iter().collect();

        if filepath.is_file() {
            return Some(filepath);
        }

        match directory.parent() {
            Some(parent) => directory = parent,
            None => return None,
        }
    }
}

fn read_markets() -> Result<HashMap<String, Market>> {
    let dir = Path::new("/tmp/fixxx-pakjekraam/config/markt");
    let mut markets: HashMap<String, Market> = HashMap::new();
    if dir.is_dir() {
        let mut i: i32 = 0;
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                match entry.file_name().into_string() {
                    Ok(file_name) => {
                        let market_id = file_name.split("-").take(1).collect();
                        let day_id: String = file_name.rsplit("-").take(1).collect();
                        markets.insert(file_name.split("-").take(1).collect(), set_market(&markets, &market_id, &day_id, &i));
                        i = i + 1;
                    }
                    Err(_) => { /* Do nothing */ }
                }
            }
        }
    }
    Ok(markets)
}

fn read_file(filename: &str) -> String {
    match File::open(String::from(filename)) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Unable to read the file");
            return contents;
        }
        Err(_error) => return String::from("{ \"error\": \"Error opening file\" }"),
    }
}

#[get("/markt/mededelingen.json")]
fn get_announcements() -> Json<Option<Announcements>> {
    let announcements: String = read_file("/tmp/fixxx-pakjekraam/config/markt/mededelingen.json");
    Json(match serde_json::from_str(&announcements) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            None
        }
    })
}

#[get("/markt/branches.json")]
fn get_branches() -> Json<Vec<Branche>> {
    let branches: String = read_file("/tmp/fixxx-pakjekraam/config/markt/branches.json");
    Json(match serde_json::from_str(&branches) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            vec![]
        }
    })
}

#[get("/markt/daysClosed.json")]
fn get_days_closed() -> Json<Vec<String>> {
    let days_closed: String = read_file("/tmp/fixxx-pakjekraam/config/markt/daysClosed.json");

    Json(match serde_json::from_str(&days_closed) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            vec![String::from("error")]
        }
    })
}

#[get("/markets.json")]
fn get_markets() -> Json<Option<HashMap<String, Market>>> {
    Json(match read_markets() {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Fail: {}", e);
            None
        }
    })
}

#[get("/markt/obstakeltypes.json")]
fn get_obstacle_types() -> Json<Vec<String>> {
    let obstacle_types: String = read_file("/tmp/fixxx-pakjekraam/config/markt/obstakeltypes.json");
    Json(match serde_json::from_str(&obstacle_types) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            vec![String::from("error")]
        }
    })
}

#[get("/markt/plaatseigenschappen.json")]
fn get_properties() -> Json<Vec<String>> {
    let properties: String =
        read_file("/tmp/fixxx-pakjekraam/config/markt/plaatseigenschappen.json");
    Json(match serde_json::from_str(&properties) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            vec![String::from("error")]
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
