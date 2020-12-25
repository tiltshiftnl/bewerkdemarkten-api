#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use git2::Repository;

mod announcement;
mod branche;
mod day;
mod obstacle;
mod properties;

fn main() {
    let url = "https://github.com/Amsterdam/fixxx-pakjekraam.git";
    match Repository::clone(url, "./fixxx-pakjekraam") {
       Ok(_repo) => println!("Repo cloned"),
       Err(e) => println!("failed to clone: {}", e),
    };
    let mut rocket = rocket::ignite();
    rocket = announcement::mount(rocket);
    rocket = branche::mount(rocket);
    rocket = day::mount(rocket);
    rocket = obstacle::mount(rocket);
    rocket = properties::mount(rocket);
    rocket.launch();
}
