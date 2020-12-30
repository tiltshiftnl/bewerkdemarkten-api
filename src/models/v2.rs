/* Import macros and others */
use crate::schema::*;

/* For beeing able to serialize */
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Market {
    pub id: i32, 
    pub name: String,
    pub abbreviation: String,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name="markets"]
pub struct NewMarket<'x> {
    pub name: &'x str,
    pub abbreviation: &'x str,
}