#[macro_use]
extern crate serde_derive;

extern crate docopt;
extern crate exit_code;
extern crate serde_yaml;
extern crate spor;

use docopt::Docopt;
use spor::anchor::Anchor;
use spor::diff::get_anchor_diff;
use spor::repository::Repository;
use spor::result::{from_str, Result};
use spor::updating::update;

const USAGE: &'static str = "
spor

Usage:
  spor init
  spor add <source-file> <offset> <width> <context-width>
  spor list <source-file>
  spor status
  spor update

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_init: bool,
    cmd_add: bool,
    cmd_list: bool,
    cmd_status: bool,
    cmd_update: bool,
    arg_source_file: String,
    arg_offset: u64,
    arg_width: u64,
    arg_context_width: u64,
}

fn init_handler() -> Result<i32> {
    std::env::current_dir()
        .map_err(|err| err.into())
        .map(|path| spor::repository::initialize(&path, None))
        .and(Ok(exit_code::SUCCESS))
}

fn add_handler(args: &Args) -> Result<i32> {
    let path = std::env::current_dir()?;
    let repo = Repository::new(&path, None)?;

    // TODO: Consider support for launching an editor when necessary.
    let metadata = match serde_yaml::from_reader(std::io::stdin()) {
        Err(err) => return Err(err.into()),
        Ok(metadata) => metadata,
    };

    let encoding = "utf-8".to_string();
    let anchor = Anchor::new(
        std::path::Path::new(&args.arg_source_file),
        args.arg_offset,
        args.arg_width,
        args.arg_context_width,
        metadata,
        encoding,
    )?;

    match repo.add(anchor) {
        Ok(_) => Ok(exit_code::SUCCESS),
        Err(err) => Err(err.into()),
    }
}

fn list_handler(args: &Args) -> Result<i32> {
    let file = std::path::Path::new(&args.arg_source_file);
    let repo = Repository::new(file, None)?;
    for anchor in &repo {
        if let Ok((_id, a)) = anchor {
            println!("{:?}", a);
        }
    }

    Ok(exit_code::SUCCESS)
}

fn status_handler(_args: &Args) -> Result<i32> {
    // TODO: Improve this output.

    let file = std::path::Path::new(".");
    let repo = Repository::new(file, None)?;
    for anchor in &repo {
        let (_, anchor) = anchor.unwrap();
        let (changed, diffs) = get_anchor_diff(&anchor)?;
        if changed {
            println!("path: {:?}", anchor.file_path);
            for diff in diffs {
                println!("\t{}", diff);
            }
        }
    }

    Ok(exit_code::SUCCESS)
}

fn update_handler(_args: &Args) -> Result<i32> {
    let file = std::path::Path::new(".");
    let repo = Repository::new(file, None)?;
    for anchor in &repo {
        let (id, anchor) = anchor.unwrap();
        let updated = update(&anchor)?;
        repo.update(id, &updated)?;
    }

    Ok(exit_code::SUCCESS)
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let result = if args.cmd_init {
        init_handler()
    } else if args.cmd_add {
        add_handler(&args)
    } else if args.cmd_list {
        list_handler(&args)
    } else if args.cmd_status {
        status_handler(&args)
    } else if args.cmd_update {
        update_handler(&args)
    } else {
        from_str("Unknown command")
    };

    let code = match result {
        Ok(code) => code,
        Err(err) => {
            println!("{}", err);
            exit_code::FAILURE
        }
    };

    std::process::exit(code)
}
