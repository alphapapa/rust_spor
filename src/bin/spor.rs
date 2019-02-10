extern crate failure; 

#[macro_use]
extern crate serde_derive;

extern crate docopt;
extern crate exit_code;
extern crate serde_yaml;
extern crate spor;

use std::path::PathBuf;

use docopt::Docopt;
use spor::alignment::smith_waterman::align;
use spor::anchor::Anchor;
use spor::diff::get_anchor_diff;
use spor::repository::{AnchorId, Repository};
use spor::updating::update;

const USAGE: &'static str = "
spor

Usage:
  spor init
  spor add <source-file> <offset> <width> <context-width>
  spor list <source-file>
  spor details <id>
  spor diff <anchor-id>
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
    cmd_details: bool,
    cmd_diff: bool,
    arg_source_file: String,
    arg_offset: u64,
    arg_width: u64,
    arg_context_width: u64,
    arg_id: String,
    arg_anchor_id: String,
}

type CommandResult = std::result::Result<(), i32>;

fn init_handler() -> CommandResult {
    let path = std::env::current_dir()
        .map_err(|_| exit_code::OS_FILE_ERROR)?;

    spor::repository::initialize(&path, None) 
        .map_err(|_| exit_code::DATA_ERROR)?;

    Ok(())
}

fn open_repo(path: &PathBuf) -> std::result::Result<Repository, i32> {
    Repository::new(&path, None)
        .map_err(|e| {
            println!("{:?}", e);
            exit_code::OS_FILE_ERROR
        })
}

fn add_handler(args: &Args) -> CommandResult {
    let path = std::env::current_dir()
        .map_err(|e| {
            println!("{:?}", e);
            exit_code::OS_FILE_ERROR
        })?;

    let repo = open_repo(&path)?;

    // TODO: Consider support for launching an editor when necessary.
    let metadata = serde_yaml::from_reader(std::io::stdin())
        .map_err(|e| {
            println!("{:?}", e);
            exit_code::DATA_ERROR
        })?;

    let encoding = "utf-8".to_string();
    let anchor = Anchor::new(
        &repo.root.join(std::path::Path::new(&args.arg_source_file)),
        args.arg_offset,
        args.arg_width,
        args.arg_context_width,
        metadata,
        encoding,
    ).map_err(|e| {
        println!("{:?}", e);
        exit_code::DATA_ERROR
    })?;

    repo.add(anchor).map_err(|e| {
        println!("{:?}", e);
        exit_code::OS_FILE_ERROR
    })?;

    Ok(())
}

fn list_handler(args: &Args) -> CommandResult {
    let file = std::path::Path::new(&args.arg_source_file);
    let repo = open_repo(&file.to_path_buf())?;
    for (id, anchor) in &repo {
        println!("{} {:?}:{} => {:?}",
            id,
            anchor.file_path(),
            anchor.context().offset(),
            anchor.metadata());
    }

    Ok(())
}

fn status_handler(_args: &Args) -> CommandResult {
    let file = std::path::Path::new(".");
    let repo = open_repo(&file.to_path_buf())?;

    for (id, anchor) in &repo {
        let diffs = get_anchor_diff(&anchor)
            .map_err(|_e| exit_code::OS_FILE_ERROR)?;

        if !diffs.is_empty() {
            println!("{} {}:{} out-of-date", 
                     id, 
                     anchor.file_path().to_string_lossy(), 
                     anchor.context().offset());
        }
    }

    Ok(())
}

fn diff_handler(args: &Args) -> CommandResult {
    let file = std::path::Path::new(".");
    let repo = open_repo(&file.to_path_buf())?;

    let anchor = repo.get(&args.arg_anchor_id)
        .or(Err(exit_code::OS_FILE_ERROR))
        .map(|a| match a {
            Some(anchor) => Ok(anchor),
            None => Err(exit_code::OS_FILE_ERROR)
        })
        .unwrap_or(Err(exit_code::OS_FILE_ERROR))?;

    let diff = get_anchor_diff(&anchor)
        .map_err(|_| exit_code::OS_FILE_ERROR)?;

    for line in diff {
        println!("{}", line);
    }

    Ok(())
}

fn update_handler(_args: &Args) -> CommandResult {
    let file = std::path::Path::new(".");

    let repo = Repository::new(file, None)
        .map_err(|_| exit_code::OS_FILE_ERROR)?;

    for (id, anchor) in &repo {
        let updated = update(&anchor, &align)
            .map_err(|e| {
                println!("{:?}", e);
                exit_code::DATA_ERROR})?;

        repo.update(id, &updated)
            .map_err(|e| {
                println!("{:?}", e);
                exit_code::OS_FILE_ERROR})?;
    };

    Ok(())
}

fn get_anchor(repo: &Repository, id_prefix: &str) -> std::result::Result<(AnchorId, Anchor), i32> {
    let mut prefixed: Vec<(AnchorId, Anchor)> = repo.into_iter()
        .filter(|(id, _anchor)| id.starts_with(id_prefix))
        .collect();

    if prefixed.len() > 1 {
        println!("Ambiguous ID specification");
        return Err(exit_code::DATA_ERROR)
    }

    match prefixed.pop() {
        Some(m) => return Ok(m),
        None => {
            println!("No anchor matching ID specification");
            return Err(exit_code::DATA_ERROR)
        }
    }
}

fn details_handler(args: &Args) -> CommandResult {
    let file = std::path::Path::new(".");
    let repo = open_repo(&file.to_path_buf())?;

    let (id, anchor) = get_anchor(&repo, &args.arg_id)?;

    print!("id: {}
path: {:?}
encoding: {}

[before]
{}
--------

[topic]
{}
--------

[after]
{}
--------

offset: {}
width: {}", 
    id, 
    anchor.file_path(), 
    anchor.encoding(),
    anchor.context().before(),
    anchor.context().topic(),
    anchor.context().after(),
    anchor.context().offset(),
    anchor.context().width(),
    );

    Ok(())
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let result = if args.cmd_init {
        init_handler() 
    } else if args.cmd_list {
        list_handler(&args)
    } else if args.cmd_status {
        status_handler(&args)
    } else if args.cmd_add {
        add_handler(&args)
    } else if args.cmd_update {
        update_handler(&args)
    } else if args.cmd_details {
        details_handler(&args)
    } else if args.cmd_diff {
        diff_handler(&args)
    } else {
        Err(exit_code::FAILURE)
    };

    std::process::exit(
        match result {
            Ok(_) => exit_code::SUCCESS,
            Err(code) => code
        });
}
