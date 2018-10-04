extern crate yaml_rust;

use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
struct Context {
    before: Vec<String>,
    line: String,
    after: Vec<String>
}

impl Context {
    fn new(context_size: usize,
           file_name: &Path,
           line_number: usize) -> Result<Context> {
        let f = File::open(file_name)?;
        let mut reader = BufReader::new(f);

        let mut buff = String::new();

        let begin_start = max(0, line_number - context_size);

        // consume up to beginning of before
        for _ in 0..begin_start {
            reader.read_line(&mut buff)?;
        }

        // read the context before
        let mut before = vec![];
        for _ in begin_start..line_number {
            let mut buff = String::new();
            reader.read_line(&mut buff)?;
            before.push(buff);
        }

        // read the line itself
        let mut line = String::new();
        reader.read_line(&mut line)?;

        // Read the after
        let mut after = vec![];
        for _ in 0..context_size {
            let mut buff = String::new();
            let size = reader.read_line(&mut buff)?;
            if size == 0 {
                break;
            }
            after.push(buff);
        }

        let context = Context {
            before: before,
            line: line,
            after: after
        };

        Ok(context)
    }
}

// struct Columns {
//     start: usize,
//     end: usize
// }

// impl Columns {
//     fn new(&self, start: usize, end: usize) {
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct Anchor {
    file_path: PathBuf,
    line_number: usize,
    columns: Option<(usize, usize)>,
    context: Context,
    metadata: serde_yaml::Value,
}

impl Anchor {
    pub fn new(context_size: usize,
               file_path: &Path,
               line_number: usize,
               metadata: serde_yaml::Value,
               columns: Option<(usize, usize)>,
               root: &Path) -> Result<Anchor> {
        let context = Context::new(context_size, &root.join(file_path), line_number)?;

        let anchor = Anchor {
            file_path: PathBuf::from(file_path),
            line_number: line_number,
            columns: columns,
            context: context,
            metadata: metadata
        };

        Ok(anchor)
    }
}
