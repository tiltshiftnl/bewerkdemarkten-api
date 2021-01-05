use bewerkdemarkten_api::models::v1::{Branche, Geography, Location, Page, Rows};
use rocket::{self, response::Debug, Data};
use rocket_contrib::json::Json;
use std::{error::Error, fs::create_dir_all, fs::remove_file, fs::write, fs::File, io, io::Read};

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
fn get_market_day_branches(market_day: String) -> Json<Option<Vec<Branche>>> {
    let market_day_branches: String = read_file(format!(
        "/tmp/fixxx-pakjekraam/config/markt/{}/branches.json",
        market_day
    ));
    Json(match serde_json::from_str(&market_day_branches) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            None
        }
    })
}

#[post(
    "/<market_day>/branches.json",
    format = "application/json",
    data = "<data>"
)]
fn post_market_day_branches(
    market_day: String,
    data: Json<Vec<Branche>>,
) -> Result<Json<String>, Box<dyn Error>> {
    let filename: String = format!(
        "/tmp/fixxx-pakjekraam/config/markt/{}/branches.json",
        market_day
    );
    let data = serde_json::to_string(&data.into_inner())?;
    create_dir_all(format!("/tmp/fixxx-pakjekraam/config/markt/{}", market_day))?;
    write(&filename, data)?;
    Ok(Json("ok".to_string()))
}

#[get("/<market_day>/geografie.json")]
fn get_market_day_geography(market_day: String) -> Json<Option<Geography>> {
    let market_day_geography: String = read_file(format!(
        "/tmp/fixxx-pakjekraam/config/markt/{}/geografie.json",
        market_day
    ));
    Json(match serde_json::from_str(&market_day_geography) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            None
        }
    })
}

#[post(
    "/<market_day>/geografie.json",
    format = "application/json",
    data = "<data>"
)]
fn post_market_day_geography(
    market_day: String,
    data: Json<Option<Geography>>,
) -> Result<Json<String>, Box<dyn Error>> {
    let filename: String = format!(
        "/tmp/fixxx-pakjekraam/config/markt/{}/geografie.json",
        market_day
    );
    let data = serde_json::to_string(&data.into_inner())?;
    create_dir_all(format!("/tmp/fixxx-pakjekraam/config/markt/{}", market_day))?;
    write(&filename, data)?;
    Ok(Json("ok".to_string()))
}

#[get("/<market_day>/locaties.json")]
fn get_market_day_locations(market_day: String) -> Json<Option<Vec<Location>>> {
    let market_day_locations: String = read_file(format!(
        "/tmp/fixxx-pakjekraam/config/markt/{}/locaties.json",
        market_day
    ));
    Json(match serde_json::from_str(&market_day_locations) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            None
        }
    })
}

#[post(
    "/<market_day>/locaties.json",
    format = "application/json",
    data = "<data>"
)]
fn post_market_day_locations(
    market_day: String,
    data: Json<Vec<Location>>,
) -> Result<Json<String>, Box<dyn Error>> {
    let filename: String = format!(
        "/tmp/fixxx-pakjekraam/config/markt/{}/locaties.json",
        market_day
    );
    let data = serde_json::to_string(&data.into_inner())?;
    create_dir_all(format!("/tmp/fixxx-pakjekraam/config/markt/{}", market_day))?;
    write(&filename, data)?;
    Ok(Json("ok".to_string()))
}

#[get("/<market_day>/markt.json")]
fn get_market_day_rows(market_day: String) -> Json<Option<Rows>> {
    let market_day_rows: String = read_file(format!(
        "/tmp/fixxx-pakjekraam/config/markt/{}/markt.json",
        market_day
    ));
    Json(match serde_json::from_str(&market_day_rows) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            Some(Rows::new())
        }
    })
}

#[post(
    "/<market_day>/markt.json",
    format = "application/json",
    data = "<data>"
)]
fn post_market_day_rows(
    market_day: String,
    data: Json<Rows>,
) -> Result<Json<String>, Box<dyn Error>> {
    let filename: String = format!(
        "/tmp/fixxx-pakjekraam/config/markt/{}/markt.json",
        market_day
    );
    let data = serde_json::to_string(&data.into_inner())?;
    create_dir_all(format!("/tmp/fixxx-pakjekraam/config/markt/{}", market_day))?;
    write(&filename, data)?;
    Ok(Json("ok".to_string()))
}

#[get("/<market_day>/paginas.json")]
fn get_market_day_pages(market_day: String) -> Json<Option<Vec<Page>>> {
    let market_day_pages: String = read_file(format!(
        "/tmp/fixxx-pakjekraam/config/markt/{}/paginas.json",
        market_day
    ));
    Json(match serde_json::from_str(&market_day_pages) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            None
        }
    })
}

#[post(
    "/<market_day>/paginas.json",
    format = "application/json",
    data = "<data>"
)]
fn post_market_day_pages(
    market_day: String,
    data: Json<Vec<Page>>,
) -> Result<Json<String>, Box<dyn Error>> {
    let filename: String = format!(
        "/tmp/fixxx-pakjekraam/config/markt/{}/paginas.json",
        market_day
    );
    let data = serde_json::to_string(&data.into_inner())?;
    create_dir_all(format!("/tmp/fixxx-pakjekraam/config/markt/{}", market_day))?;
    write(&filename, data)?;
    Ok(Json("ok".to_string()))
}

#[get("/<market_day>/download/pdf")]
fn get_market_day_pdf(market_day: String) -> Option<File> {
    let filename: String = format!(
        "/tmp/fixxx-pakjekraam/dist/pdf/{}-{}.pdf",
        "kaart",
        market_day.to_string()
    );
    println!("{}", filename);
    File::open(&filename).ok()
}

#[post(
    "/<market_day>/upload/pdf",
    format = "multipart/form-data",
    data = "<data>"
)]
fn post_market_day_pdf(market_day: String, data: Data) -> Result<String, Debug<io::Error>> {
    let filename: String = format!(
        "/tmp/fixxx-pakjekraam/dist/pdf/{}-{}.pdf",
        "kaart",
        market_day.to_string()
    );
    data.stream_to_file(filename)
        .map(|n| n.to_string())
        .map_err(Debug)
}

#[delete("/<market_day>/delete/pdf")]
fn delete_market_day_pdf(market_day: String) -> Result<String, Debug<io::Error>> {
    let filename: String = format!(
        "/tmp/fixxx-pakjekraam/dist/pdf/{}-{}.pdf",
        "kaart",
        market_day.to_string()
    );
    remove_file(&filename)
        .map(|_| "ok".to_string())
        .map_err(Debug)
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount(
        "/api/v1/markt",
        routes![
            delete_market_day_pdf,
            get_market_day_branches,
            get_market_day_geography,
            get_market_day_locations,
            get_market_day_rows,
            get_market_day_pages,
            get_market_day_pdf,
            post_market_day_pdf,
            post_market_day_branches,
            post_market_day_geography,
            post_market_day_locations,
            post_market_day_rows,
            post_market_day_pages,
        ],
    )
}
