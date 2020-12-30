pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Market, NewMarket};

pub fn create_market<'a>(conn: &PgConnection, name: &'a str) -> Market {
    use schema::markets;

    let new_market = NewMarket { name };

    diesel::insert_into(markets::table)
        .values(&new_market)
        .get_result(conn)
        .expect("Error saving new market")
}
