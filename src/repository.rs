extern crate serde_yaml;
extern crate uuid;

use std::fs::{DirBuilder, write};
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

use anchor::Anchor;

pub struct Repository {
    root: PathBuf,
    spor_dir: PathBuf
}

impl Repository {
    pub fn new(path: &Path, spor_dir: Option<&Path>) -> Result<Repository>
    {
        let path = PathBuf::from(path).canonicalize()?;
        let spor_dir = match spor_dir {
            None => PathBuf::from(".spor"),
            Some(dir) => PathBuf::from(dir)
        };
        let spor_dir = path.join(spor_dir);

        if !spor_dir.exists() {
            return Err(
                Error::new(
                    ErrorKind::NotFound,
                    format!("spor directory not found: {:?}", spor_dir)));
        }

        let repo = Repository {
            root: path,
            spor_dir: spor_dir
        };
        Ok(repo)
    }

    pub fn add(&self,
           metadata: serde_yaml::Value,
           file_path: &Path,
           line_number: usize,
           columns: Option<(usize, usize)>)
           -> Result<uuid::Uuid>
    {
        let anchor = Anchor::new(3, file_path, line_number, metadata, columns, &self.root)?;
        let anchor_id = uuid::Uuid::new_v4();
        let anchor_path = self.anchor_path(anchor_id);

        let s = match serde_yaml::to_string(&anchor) {
            Err(info) => return Err(
                Error::new(ErrorKind::InvalidData, info)),
            Ok(s) => s
        };
        write(anchor_path, s)?;

        Ok(anchor_id)
    }

    /// Absolute path to the data file for `anchor_id`.
    fn anchor_path(&self, anchor_id: uuid::Uuid) -> PathBuf {

        let file_name = format!("{}.yml", anchor_id);
        let path = self.spor_dir.join(file_name);
        assert!(path.is_absolute());
        path
    }

    // get by id
    // update
    // remove
    // iterate
    // items

}

/// Initialize a spor repository in `path` if one doesn't already exist.
pub fn initialize(path: &Path, spor_dir: Option<&Path>) -> Result<()> {
    let spor_dir = match spor_dir {
        None => Path::new(".spor"),
        Some(d) => d
    };

    let spor_path = path.join(spor_dir);

    if spor_path.exists() {
        return Err(
            Error::new(
                ErrorKind::AlreadyExists,
                format!(
                    "spor directory already exists: {}",
                    spor_path.to_string_lossy())));
    }

    let mut builder = DirBuilder::new();
    builder.recursive(true);
    builder.create(spor_path)
}
