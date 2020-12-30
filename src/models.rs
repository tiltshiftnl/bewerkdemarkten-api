use super::schema::markets;

#[derive(Queryable)]
pub struct Market {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "markets"]
pub struct NewMarket<'a> {
    pub name: &'a str,
}
