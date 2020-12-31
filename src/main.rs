#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate dotenv;

use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;

use dotenv::dotenv;
use git2::Repository;
use rocket::http::Method;
use rocket::response::content::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::env;
mod generic;
mod market;

#[catch(500)]
fn not_found() -> Json<&'static str> {
    Json("{\"error\": \"not implemented\"}")
}


fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

fn main() {
    dotenv().ok();
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    match env::var("GIT_REPOSITORY") {
        Ok(val) => {
            match Repository::clone(&val, "/tmp/fixxx-pakjekraam") {
                Ok(_repo) => println!("{} cloned", val),
                Err(e) => println!("Failed to clone: {}", e),
            };
        }
        Err(e) => println!("couldn't interpret {}: {}", "GIT_REPOSITORY", e),
    };

    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

    database_config.insert("url", Value::from(database_url()));
    database_config.insert("pool_size", Value::from(20));
    databases.insert("bewerkdemarkten_db", Value::from(database_config));

    let config = Config::build(Environment::Development)
        .extra("databases", databases)
        .finalize()
        .unwrap();

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error could not create CORS fairing");

    let mut rocket = rocket::custom(config);
    rocket = generic::v1::mount(rocket);
    rocket = market::v1::mount(rocket);
    rocket = market::v2::mount(rocket);
    rocket
        .register(catchers![not_found])
        .attach(cors)
        .launch();
}
