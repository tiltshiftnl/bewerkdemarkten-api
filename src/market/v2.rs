use bewerkdemarkten_api::models::v2::{Market};
use rocket_contrib::json::Json;
use crate::DbConn;

#[get("/markets")]
pub fn index(connection: DbConn) -> Json<Vec<Market>> {
    let markets = Market::all(&connection);
    Json(markets)
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount(
        "/api/v2",
        routes![
            index,
        ],
    )
}
