#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

//use git2::Repository;
//use rocket::response::content;

//fn main() {
// let point = Point { x: 1, y: 2}
// let url = "https://github.com/Amsterdam/fixxx-pakjekraam.git";
// match Repository::clone(url, "./fixxx-pakjekraam") {
//     Ok(_repo) => println!("Repo cloned"),
//     Err(_e) => println!("Repo exists"),
// };
// rocket::ignite().mount("/api", routes![get_branches, get_days_closed, get_announcements, get_obstacle_types]).launch();
//}
mod announcement;
mod branche;
mod day;
mod obstacle;
mod properties;

fn main() {
    let mut rocket = rocket::ignite();
    rocket = announcement::mount(rocket);
    rocket = branche::mount(rocket);
    rocket = day::mount(rocket);
    rocket = obstacle::mount(rocket);
    rocket = properties::mount(rocket);
    rocket.launch();
}
