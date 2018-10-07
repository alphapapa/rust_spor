extern crate glob;
extern crate serde_yaml;
extern crate uuid;

use std::fs::{DirBuilder, File};
use std::io;
use std::path::{Path, PathBuf};

use anchor::Anchor;
use result::{from_str, Result};

pub type AnchorId = String;

fn new_anchor_id() -> AnchorId {
    format!("{}", uuid::Uuid::new_v4())
}

#[derive(Debug)]
pub struct Repository {
    pub root: PathBuf,
    pub spor_dir: PathBuf
}

fn write_anchor(anchor_path: &Path, anchor: &Anchor) -> io::Result<()> {
    let f = File::create(anchor_path)?;
    let writer = io::BufWriter::new(f);
    match serde_yaml::to_writer(writer, &anchor) {
        Err(info) => return Err(
            io::Error::new(
                io::ErrorKind::InvalidData, info)),
        Ok(s) => Ok(s)
    }
}

fn read_anchor(anchor_path: &Path) -> io::Result<Anchor> {
    let f = File::open(anchor_path)?;
    let reader = io::BufReader::new(f);
    match serde_yaml::from_reader(reader) {
        Err(info) => return Err(
            io::Error::new(
                io::ErrorKind::InvalidData, info)),
        Ok(a) => Ok(a)
    }
}

/// Search for a spor repo containing `path`.
///
/// This searches for `spor_dir` in directories dominating `path`. If a
/// directory containing `spor_dir` is found, then that directory is returned.
///
/// Returns: The dominating directory containing `spor_dir`.
fn find_root_dir(path: &Path, spor_dir: &Path) -> io::Result<Option<PathBuf>> {
    let p = PathBuf::from(path).canonicalize()?;

    for ancestor in p.ancestors() {
        let data_dir = ancestor.join(spor_dir);
        if data_dir.exists() && data_dir.is_dir() {
            return Ok(Some(ancestor.to_path_buf()));
        }
    }

    Ok(None)
}

impl Repository {

    /// Find the repository directory for the file `path` and return a
    /// `Repository` for it.
    pub fn new(path: &Path, spor_dir: Option<&Path>) -> io::Result<Repository>
    {
        let spor_dir = match spor_dir {
            None => PathBuf::from(".spor"),
            Some(dir) => PathBuf::from(dir)
        };


        let root = find_root_dir(path, &spor_dir)?;
        let root = match root {
            None => return Err(
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("spor repository not found for {:?}", path))),
            Some(p) => p
        };

        let spor_dir = root.join(spor_dir);
        assert!(spor_dir.exists(),
                "spor-dir not found after find_root_dir succeeded!");

        let repo = Repository {
            root: root,
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

        if anchor_path.exists() {
            return Err(
                io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!("{:?} already exists", anchor_path)));
        }

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

    // get by id
    // update
    // remove
    // iterate
    // items

}

impl <'a> IntoIterator for &'a Repository {
    type Item = <RepositoryIterator as Iterator>::Item;
    type IntoIter = RepositoryIterator;

    fn into_iter(self) -> Self::IntoIter {
        RepositoryIterator::new(&self.spor_dir)
    }
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
    type Item = Result<(AnchorId, Anchor)>;

    fn next(&mut self) -> Option<Self::Item> {
        let glob_result = self.anchor_files.next()?;
        let anchor_path = match glob_result {
            Ok(p) => p,
            Err(err) => return Some(Err(err.into()))
        };

        let anchor_id = match anchor_path.file_stem() {
            Some(id) => id,
            None => return Some(from_str(&format!("Unable to get file stem for {:?}", anchor_path)))
        };

        let anchor_id = match anchor_id.to_str() {
            Some(s) => String::from(s),
            None => return Some(from_str(&format!("Error converting {:?} to string", anchor_id)))
        };

        let anchor = match read_anchor(&anchor_path) {
            Ok(anchor) => anchor,
            Err(err) => return Some(from_str(&format!("{:?}", err)))
        };

        Some(Ok((anchor_id, anchor)))
    }
}

/// Initialize a spor repository in `path` if one doesn't already exist.
pub fn initialize(path: &Path, spor_dir: Option<&Path>) -> Result<()> {
    let spor_dir = spor_dir.unwrap_or(Path::new(".spor"));

    let spor_path = path.join(spor_dir);

    if spor_path.exists() {
        return from_str(&format!(
            "spor directory already exists: {}",
            spor_path.to_string_lossy()));
    }

    let mut builder = DirBuilder::new();
    builder.recursive(true);
    builder.create(spor_path)
        .or_else(|e| Err(e.into()))
}
