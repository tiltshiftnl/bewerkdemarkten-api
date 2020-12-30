//use bewerkdemarkten_api::models::v2::{Market};
//use bewerkdemarkten_api::schema::*;

/* List our inserted markets */
#[get("/")]
pub fn list() -> String {
    String::from("Ok")
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount(
        "/api/v2",
        routes![
            list,
        ],
    )
}