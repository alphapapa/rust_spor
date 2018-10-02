#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate spor;

use docopt::Docopt;
use spor::repository;

const USAGE: &'static str = "
spor

Usage:
  spor init
  spor add

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_init: bool,
    cmd_add: bool,
    // flag_speed: isize,
    // flag_drifting: bool,
    // arg_name: Vec<String>,
    // arg_x: Option<i32>,
    // arg_y: Option<i32>,
    // cmd_ship: bool,
    // cmd_mine: bool,
}

fn init_handler() -> std::io::Result<()> {
    let path = std::env::current_dir()?;
    spor::repository::initialize(&path, None)
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_init {
        if let Err(err) = init_handler() {
            println!("{}", err);
        }
    }
    else {
        println!("Unknown command");
    };
}
