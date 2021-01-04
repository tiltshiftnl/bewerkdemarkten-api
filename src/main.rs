#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
//#[macro_use] extern crate rocket_contrib;

extern crate dotenv;

use dotenv::dotenv;
use git2::Repository;
//use rocket::config::{Config, Environment, Value};
//use rocket_contrib::databases::diesel;
use rocket::http::Method;
use rocket::response::content::Json;
use rocket_cors::{AllowedOrigins}
;
//use std::collections::HashMap;
use std::env;

mod generic;
mod market;

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

fn main() {
    dotenv().ok();

    // Repository
    match env::var("GIT_REPOSITORY") {
        Ok(val) => {
            match Repository::clone(&val, "/tmp/fixxx-pakjekraam") {
                Ok(_repo) => println!("{} cloned", val),
                Err(e) => println!("Failed to clone: {}", e),
            };
        }
        Err(e) => println!("couldn't interpret {}: {}", "GIT_REPOSITORY", e),
    };

    // Cors

    let allowed_origins =
        AllowedOrigins::some_exact(&["http://localhost:3000","https://bewerkdemarkten.tiltshiftapps.nl"]);
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Options]
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
    let mut rocket = rocket::ignite(); //rocket::custom(config);
    rocket = generic::v1::mount(rocket);
    rocket = market::v1::mount(rocket);
    //rocket = market::v2::mount(rocket);
    rocket
        // Mount the routes to catch all the OPTIONS pre-flight requests
        //.mount("/", rocket_cors::catch_all_options_routes())
        .register(catchers![not_found, server_error])
        .attach(cors)
        .launch();
}
