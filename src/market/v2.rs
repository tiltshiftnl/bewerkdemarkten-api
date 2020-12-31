//use bewerkdemarkten_api::models::v2::{InsertableMarket, Market};
use bewerkdemarkten_api::models::v2::{Market};
use bewerkdemarkten_api::schema::markets;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;

use rocket::http::Status;
use rocket_contrib::json::Json;
use crate::DbConn;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Market>> {
    markets::table.load::<Market>(connection)
}

// pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Market> {
//     markets::table.find(id).get_result::<Market>(connection)
// }

// pub fn insert(market: Market, connection: &PgConnection) -> QueryResult<Market> {
//     diesel::insert_into(markets::table)
//         .values(&InsertableMarket::from_market(market))
//         .get_result(connection)
// }

// pub fn update(id: i32, market: Market, connection: &PgConnection) -> QueryResult<Market> {
//     diesel::update(markets::table.find(id))
//         .set(&market)
//         .get_result(connection)
// }

// pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
//     diesel::delete(markets::table.find(id)).execute(connection)
// }

#[get("/")]
fn get_all(connection: DbConn) -> Result<Json<Vec<Market>>, Status> {
    all(&connection)
        .map(|market| Json(market))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/api/v2", routes![get_all,])
}