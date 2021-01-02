#![allow(proc_macro_derive_resolution_fallback)]
use ::diesel::pg::PgConnection;
use ::diesel::prelude::*;

/* Import macros and others */
use crate::schema::markets;
use crate::schema::markets::dsl::markets as all_markets;

/* For beeing able to serialize */
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize, AsChangeset)]
pub struct Market {
    pub id: i32,
    pub name: String,
    pub abbreviation: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Serialize)]
#[table_name = "markets"]
pub struct InsertableMarket {
    pub name: String,
    pub abbreviation: String,
}

impl InsertableMarket {
    pub fn from_market(market: Market) -> InsertableMarket {
        InsertableMarket {
            name: market.name,
            abbreviation: market.abbreviation,
        }
    }
}

impl Market {
    pub fn all(conn: &PgConnection) -> Vec<Market> {
        match all_markets.order(markets::id.desc()).load::<Market>(conn) {
            Ok(markets) => markets,
            Err(_) => {
                println!("Unable to load markets");
                Vec::new()
            }
        }
    }
}
