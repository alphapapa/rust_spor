#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate docopt;
extern crate spor;

use docopt::Docopt;
use spor::repository::Repository;
use std::io;

const USAGE: &'static str = "
spor

Usage:
  spor init
  spor add <source-file> <line-number> [<begin-offset> <end-offset>]
  spor list <source-file>

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_init: bool,
    cmd_add: bool,
    cmd_list: bool,
    arg_source_file: String,
    arg_line_number: usize,
    arg_begin_offset: Option<usize>,
    arg_end_offset: Option<usize>
    // flag_speed: isize,
    // flag_drifting: bool,
    // arg_name: Vec<String>,
    // arg_x: Option<i32>,
    // arg_y: Option<i32>,
    // cmd_ship: bool,
    // cmd_mine: bool,
}

fn init_handler() -> io::Result<()> {
    let path = std::env::current_dir()?;
    spor::repository::initialize(&path, None)
}

fn add_handler(args: &Args) -> std::io::Result<()> {
    let path = std::env::current_dir()?;
    let repo = Repository::new(&path, None)?;

    let columns = match args.arg_begin_offset {
        Some(begin_offset) => {
            let end_offset = args.arg_end_offset.expect("Either both or neither of offsets must be set.");
            Some((begin_offset, end_offset))
        },
        None => None
    };

    // TODO: Consider support for launching an editor when necessary.
    let metadata = match serde_yaml::from_reader(std::io::stdin()) {
        Err(err) => return Err(
            io::Error::new(
                io::ErrorKind::InvalidInput, format!("{:?}", err))),
        Ok(metadata) => metadata
    };

    match repo.add(
        metadata,
        std::path::Path::new(&args.arg_source_file),
        args.arg_line_number,
        columns)
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}

fn list_handler(args: &Args) -> io::Result<()> {
    let file = std::path::Path::new(&args.arg_source_file);
    let repo = Repository::new(file, None)?;
    for anchor in repo {
        if let Ok((_id, a)) = anchor {
            println!("{:?}", a);
        }
    }

    Ok(())
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
    else if args.cmd_add {
        if let Err(err) = add_handler(&args) {
            println!("{}", err);
        }
    }
    else if args.cmd_list {
        if let Err(err) = list_handler(&args) {
            println!("{}", err);
        }
    }
    else {
        println!("Unknown command");
    };
}
