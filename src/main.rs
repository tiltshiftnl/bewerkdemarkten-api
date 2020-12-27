#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use git2::Repository;

mod generic;
mod market;

fn main() {
    let url = "https://github.com/Amsterdam/fixxx-pakjekraam.git";
    match Repository::clone(url, "/tmp/fixxx-pakjekraam") {
       Ok(_repo) => println!("Repo cloned"),
       Err(e) => println!("Failed to clone: {}", e),
    };
    let mut rocket = rocket::ignite();
    rocket = generic::mount(rocket);
    rocket = market::mount(rocket);
    rocket.launch();
}
