#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
//#[macro_use] extern crate rocket_contrib;

//use rocket::config::{Config, Environment, Value};
//use rocket_contrib::databases::diesel;
use rocket::{Rocket, http::Method::{Get, Post, Delete, Options}, response::content::Json};
use rocket_cors::{AllowedOrigins, CorsOptions}
;
//use std::collections::HashMap;


mod generic;
mod market;
mod git;
// #[database("bewerkdemarkten_db")]
// pub struct DbConn(pub diesel::PgConnection);

#[catch(500)]
fn server_error() -> Json<&'static str> {
    Json("{\"error\": \"not implemented\"}")
}

#[catch(404)]
fn not_found() -> Json<&'static str> {
    Json("{\"error\": \"not found\"}")
}

// fn database_url() -> String {
//     env::var("DATABASE_URL").expect("DATABASE_URL must be set")
// }

fn rocket() -> Rocket {
    

    git::update_repository();

    // Cors
    let allowed_origins =
        AllowedOrigins::some_exact(&["http://localhost:3000","https://bewerkdemarkten.tiltshiftapps.nl"]);
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Get, Post, Delete, Options]
            .into_iter()
            .map(From::from)
            .collect(),
        fairing_route_base: "/api".to_string(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error could not create CORS fairing");

    // Database
    // let mut database_config = HashMap::new();
    // let mut databases = HashMap::new();
    // database_config.insert("url", Value::from(database_url()));
    // database_config.insert("pool_size", Value::from(20));
    // databases.insert("bewerkdemarkten_db", Value::from(database_config));

    // let config = Config::build(Environment::Development)
    //     .extra("databases", databases)
    //     .finalize()
    //     .unwrap();

    // Start
    let mut rocket = rocket::ignite();
    rocket = generic::v1::mount(rocket);
    rocket = market::v1::mount(rocket);
    //rocket = market::v2::mount(rocket);
    rocket
        // Mount the routes to catch all the OPTIONS pre-flight requests
        //.mount("/", rocket_cors::catch_all_options_routes())
        .register(catchers![not_found, server_error])
        .attach(cors)
}

fn main() {
    rocket().launch();
}
