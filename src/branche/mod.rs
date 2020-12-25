use rocket::{self};
use rocket::response::content;
use std::fs::File;
use std::io::Read;

fn read_file(filename: String) -> String {
    match File::open(filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Unable to read the file");
            return contents;
        }
        Err(_error) => return "{ \"error\": \"Error opening file\" }".to_string(),
    }
}

#[get("/branches.json")]
fn get_branches() -> content::Json<String> {
    let result: String = read_file("fixxx-pakjekraam/config/markt/branches.json".to_string());
    content::Json(result)
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/api", routes![get_branches])
}
