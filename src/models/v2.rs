#![allow(proc_macro_derive_resolution_fallback)]
/* Import macros and others */
use crate::schema::*;

/* For beeing able to serialize */
use serde::{Serialize};

#[derive(Debug, Queryable, Serialize, AsChangeset)]
pub struct Market {
    pub id: i32, 
    pub name: String,
    pub abbreviation: String,
}

#[derive(Insertable, Serialize)]
#[table_name="markets"]
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