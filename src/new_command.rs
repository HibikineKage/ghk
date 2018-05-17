extern crate oauth2;
use std::io;
use self::oauth2::Config;

/*let mut config = Config::new("client_id", "client_secret", "http://authorize", "token")
.add_scope("read")
.add_scope("write");*/
pub fn new_repository(name: &String) {
    print!("Input name (*required) : ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    print!("Input description [optional]: ");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");
}
