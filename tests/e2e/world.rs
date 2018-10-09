extern crate cucumber_rust;
extern crate tempdir;

use std::path::PathBuf;
use tempdir::TempDir;

pub struct World {
    start_dir: PathBuf,
    pub repo_dir: PathBuf,
    temp_dir: TempDir,
    pub executable: PathBuf,
}

impl cucumber_rust::World for World {}

impl std::default::Default for World {
    fn default() -> World {
        let dir = TempDir::new("spor_cucumber_tests")
            .expect("Unable to create temporary working directory");

        let cwd = std::env::current_dir().expect("Unable to get current directory");

        let executable = std::env::current_exe().expect("Unable to get test executable")
            .parent().expect("Unable to get executable directory")
            .parent().expect("Unable to get parent of executable directory")
            .join("spor");

        let world = World {
            executable: executable,
            start_dir: cwd,
            repo_dir: dir.path().to_path_buf(),
            temp_dir: dir,
        };

        std::env::set_current_dir(&world.repo_dir)
            .expect("Unable to switch to temp dir for testing");

        world
    }
}

impl std::ops::Drop for World {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.start_dir)
            .expect("Unable to switch back to starting directory after test");
    }
}
