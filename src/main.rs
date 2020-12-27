#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate dotenv;

use git2::Repository;
use dotenv::dotenv;
use std::env;
mod generic;
mod market;

fn main() {
    
    dotenv().ok();

    match env::var("GIT_REPOSITORY") {
        Ok(val) => {
            match Repository::clone(&val, "/tmp/fixxx-pakjekraam") {
                Ok(_repo) => println!("{} cloned", val),
                Err(e) => println!("Failed to clone: {}", e),
             };
        },
        Err(e) => println!("couldn't interpret {}: {}", "GIT_REPOSITORY", e),
    };

    let mut rocket = rocket::ignite();
    rocket = generic::mount(rocket);
    rocket = market::mount(rocket);
    rocket.launch();
}
