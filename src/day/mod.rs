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


#[get("/test")]
fn get_test() -> content::Json<String> {
    //let array: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    
    let result: String = read_file("/tmp/fixxx-pakjekraam/config/markt/daysClosed.json".to_string());
    println!("{}", result);
    let array: Vec<String> = match serde_json::from_str(&result) {
        Ok(result) => result,
        Err(e) => {
            println!("Fail: {}", e);
            vec!["error".to_string()]
        },
    };

    let result: String = match serde_json::to_string(&array) {
        Ok(result) => result,
        Err(_err) => "error".to_string(),
    };
    content::Json(result)
}

#[get("/daysClosed.json")]
fn get_days_closed() -> content::Json<String> {
    let result: String = read_file("fixxx-pakjekraam/config/markt/daysClosed.json".to_string());
    content::Json(result)
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/api", routes![get_days_closed, get_test])
}
