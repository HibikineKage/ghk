extern crate oauth2;
use std::io;
use oauth2::Config;

let mut config = Config::new("client_id", "client_secret", "http://authorize")
pub fn new_repository(name: &String) {
    println!("Input name (*required) : ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Input description [optional]: ");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");
}
