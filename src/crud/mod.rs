use rocket::{self};
use rocket_contrib::json::Json;

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount(
        "/api/v2",
        routes![
            post_new_market,
            get_obstacle_types,
            get_properties,
            get_branches,
            get_announcements,
            get_markets,
        ],
    )
}