extern crate docopt;
#[macro_use]
extern crate serde_derive;
mod new_command;

use docopt::Docopt;

const USAGE: &'static str = "
GitHucker {version}.

Usage:
  ghk new [<name>]
  ghk (-h | --help)

Options:
  -h --help  Show this on screen.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_name: Vec<String>,
    cmd_new: bool,
}
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    if args.cmd_new {
        new_command::new_repository(&"".to_string());
    }
}
