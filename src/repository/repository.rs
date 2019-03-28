extern crate glob;
extern crate serde;
extern crate uuid;

use std::fs::DirBuilder;
use std::io;
use std::path::{Path, PathBuf};

use anchor::Anchor;
use super::serialization::{read_anchor, write_anchor};

pub type AnchorId = String;

fn new_anchor_id() -> AnchorId {
    format!("{}", uuid::Uuid::new_v4())
}

#[derive(Debug)]
pub struct Repository {
    pub root: PathBuf,
    spor_dir: PathBuf,
}

impl Repository {
    /// Find the repository directory for the file `path` and return a
    /// `Repository` for it.
    pub fn new(path: &Path, spor_dir: Option<&Path>) -> io::Result<Repository> {
        let spor_dir = PathBuf::from(spor_dir.unwrap_or(&PathBuf::from(".spor")));

        find_root_dir(path, &spor_dir)
            .map(|root_dir| {
                assert!(
                    root_dir.join(&spor_dir).exists(),
                    "spor-dir not found after find_root_dir succeeded!"
                );

                Repository {
                    root: root_dir,
                    spor_dir: spor_dir,
                }
            })
    }

    pub fn spor_dir(&self) -> PathBuf {
        self.root.join(&self.spor_dir)
    }

    pub fn add(
        &self,
        anchor: Anchor,
    ) -> io::Result<AnchorId> {
        let anchor_id = new_anchor_id();
        let anchor_path = self.anchor_path(&anchor_id);

        if anchor_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("{:?} already exists", anchor_path),
            ));
        }

        write_anchor(&anchor_path, &anchor, &self.root)?;

        Ok(anchor_id)
    }

    pub fn update(
        &self,
        anchor_id: AnchorId,
        anchor: &Anchor
    ) -> io::Result<()> {
        let anchor_path = self.anchor_path(&anchor_id);
        if !anchor_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{:?} does not exist", anchor_path)
            ));
        }

        write_anchor(&anchor_path, &anchor, &self.root)?;

        Ok(())
    }

    /// Absolute path to the data file for `anchor_id`.
    fn anchor_path(&self, anchor_id: &AnchorId) -> PathBuf {
        let file_name = format!("{}.yml", anchor_id);
        let path = self.spor_dir().join(file_name);
        assert!(path.is_absolute());
        path
    }

    pub fn get(&self, anchor_id: &AnchorId) -> io::Result<Option<Anchor>> {
        let path = self.anchor_path(anchor_id);
        match read_anchor(&path, &self.root) {
            Err(err) => {
                match err.kind() {
                    io::ErrorKind::NotFound => {
                        Ok(None)
                    }        
                    _ => Err(err)
                }
            }
            Ok(anchor) => Ok(Some(anchor))
        }
    }

    // get by id
    // update
    // remove
    // iterate
    // items
}



/// Initialize a spor repository in `path` if one doesn't already exist.
pub fn initialize(path: &Path, spor_dir: Option<&Path>) -> io::Result<()> {
    let spor_dir = spor_dir.unwrap_or(Path::new(".spor"));

    let spor_path = path.join(spor_dir);

    if spor_path.exists() {
        Err(io::Error::new(io::ErrorKind::AlreadyExists, 
                           "spor directory already exists"))
    } else {
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        builder.create(spor_path)
    }
}

/// Search for a spor repo containing `path`.
///
/// This searches for `spor_dir` in directories dominating `path`. If a
/// directory containing `spor_dir` is found, then that directory is returned.
///
/// Returns: The dominating directory containing `spor_dir`.
fn find_root_dir(path: &Path, spor_dir: &Path) -> io::Result<PathBuf> {
    PathBuf::from(path)
        .canonicalize()
        .map(|p| {
            p.ancestors()
                .into_iter()
                .map(|a| (a, a.join(spor_dir)))
                .filter(|(_a, d)| d.exists() && d.is_dir())
                .map(|(a, _d)| PathBuf::from(a))
                .next()
        }).map(|a| PathBuf::from(a.unwrap()))
}
