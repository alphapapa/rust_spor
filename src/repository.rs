extern crate glob;
extern crate serde_yaml;
extern crate uuid;

use std::fs::{DirBuilder, File};
use std::io;
use std::path::{Path, PathBuf};

use anchor::Anchor;

type AnchorId = String;

fn new_anchor_id() -> AnchorId {
    // TODO: Is there a more direct to_str() or something?
    format!("{}", uuid::Uuid::new_v4())
}

pub struct Repository {
    root: PathBuf,
    spor_dir: PathBuf
}

fn write_anchor(anchor_path: &PathBuf, anchor: &Anchor) -> io::Result<()> {
    let f = File::open(anchor_path)?;
    let writer = io::BufWriter::new(f);
    match serde_yaml::to_writer(writer, &anchor) {
        Err(info) => return Err(
            io::Error::new(
                io::ErrorKind::InvalidData, info)),
        Ok(s) => Ok(s)
    }
}

fn read_anchor(anchor_path: &PathBuf) -> io::Result<Anchor> {
    let f = File::open(anchor_path)?;
    let reader = io::BufReader::new(f);
    match serde_yaml::from_reader(reader) {
        Err(info) => return Err(
            io::Error::new(
                io::ErrorKind::InvalidData, info)),
        Ok(a) => Ok(a)
    }
}

impl Repository {
    pub fn new(path: &Path, spor_dir: Option<&Path>) -> io::Result<Repository>
    {
        let path = PathBuf::from(path).canonicalize()?;
        let spor_dir = match spor_dir {
            None => PathBuf::from(".spor"),
            Some(dir) => PathBuf::from(dir)
        };
        let spor_dir = path.join(spor_dir);

        if !spor_dir.exists() {
            return Err(
                io::Error::new(
                    io::ErrorKind::NotFound,
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
           -> io::Result<AnchorId>
    {
        let anchor = Anchor::new(3, file_path, line_number, metadata, columns, &self.root)?;
        let anchor_id = new_anchor_id();
        let anchor_path = self.anchor_path(&anchor_id);

        write_anchor(&anchor_path, &anchor)?;

        Ok(anchor_id)
    }

    /// Absolute path to the data file for `anchor_id`.
    fn anchor_path(&self, anchor_id: &AnchorId) -> PathBuf {
        let file_name = format!("{}.yml", anchor_id);
        let path = self.spor_dir.join(file_name);
        assert!(path.is_absolute());
        path
    }

    pub fn iter(&self) -> RepositoryIterator
    {
        RepositoryIterator::new(&self.spor_dir)
    }

    // get by id
    // update
    // remove
    // iterate
    // items

}

pub struct RepositoryIterator {
    anchor_files: glob::Paths
}

impl RepositoryIterator {
    fn new(spor_dir: &PathBuf) -> RepositoryIterator {
        let glob_path = spor_dir.join("**/*.yml");

        // TODO: Probably shouldn't be using expect. Clean up the API.
        let pattern = glob_path.to_str()
            .expect(format!("Unable to stringify path {:?}. Invalid utf-8?",
                            glob_path).as_str());

        let matches = glob::glob(pattern)
            .expect("Unexpected glob failure.");

        RepositoryIterator {
            anchor_files: matches
        }
    }
}

impl Iterator for RepositoryIterator {
    type Item = Result<(AnchorId, Anchor), String>;

    fn next(&mut self) -> Option<Self::Item> {
        let glob_result = self.anchor_files.next()?;
        let anchor_path = match glob_result {
            Ok(p) => p,
            Err(err) => return Some(Err(format!("{:?}", err)))
        };

        let anchor_id = match anchor_path.file_stem() {
            Some(id) => id,
            None => return Some(Err(format!("Unable to get file stem for {:?}", anchor_path)))
        };

        let anchor_id = match anchor_id.to_str() {
            Some(s) => String::from(s),
            None => return Some(Err(format!("Error converting {:?} to string", anchor_id)))
        };

        let anchor = match read_anchor(&anchor_path) {
            Ok(anchor) => anchor,
            Err(err) => return Some(Err(format!("{:?}", err)))
        };

        Some(Ok((anchor_id, anchor)))
    }
}

/// Initialize a spor repository in `path` if one doesn't already exist.
pub fn initialize(path: &Path, spor_dir: Option<&Path>) -> io::Result<()> {
    let spor_dir = match spor_dir {
        None => Path::new(".spor"),
        Some(d) => d
    };

    let spor_path = path.join(spor_dir);

    if spor_path.exists() {
        return Err(
            io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "spor directory already exists: {}",
                    spor_path.to_string_lossy())));
    }

    let mut builder = DirBuilder::new();
    builder.recursive(true);
    builder.create(spor_path)
}
