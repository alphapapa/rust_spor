#[macro_use]
extern crate cucumber_rust;
extern crate tempdir;

use std::path::PathBuf;
use tempdir::TempDir;

pub struct World {
    start_dir: PathBuf,
    repo_dir: PathBuf,
    _temp_dir: TempDir,
    executable: PathBuf,
}

impl cucumber_rust::World for World {}

impl std::default::Default for World {
    fn default() -> World {
        let dir = TempDir::new("spor_cucumber_tests")
            .expect("Unable to create temporary working directory");

        let cwd = std::env::current_dir().expect("Unable to get current directory");

        let world = World {
            // TODO:  This is kludgy. What's the correct way to find the spor executable?
            executable: cwd.join("target/debug/spor"),
            start_dir: cwd,
            repo_dir: dir.path().to_path_buf(),
            _temp_dir: dir,
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

mod example_steps {
    use std::fs;
    use std::io::Write;
    use std::process::{Command, Stdio};

    steps!(::World => {

        given "I initialize a repository" |world, _step| {
            Command::new(&world.executable)
                .arg("init")
                .output()
                .expect("failed to execute spor");
        };

        given regex r"^I create the source file (.+)$" (String) |world, filename, _step| {
            let source_file = world.repo_dir.join(filename);
            let code = "def func():
    x = 1
    y = 2
    z = 3
    return x + y + z";
            fs::write(source_file, code)
                .expect("unable to write code to test file");
        };

        when regex r"^I modify (.+)$" (String) |world, filename, _step| {
            let source_file = world.repo_dir.join(filename);
            let code = fs::read_to_string(&source_file)
                .expect("Unable to read source file");
            let code = String::from("# a comment\n") + &code;
            fs::write(source_file, code)
                .expect("unable to write code to test file");
        };

        then "a repo data directory exists" |world, _step| {
            assert!(world.repo_dir.join(".spor").exists());
        };

        when regex r"^I create a new anchor for (.+) at line (\d+)$" (String, usize) |world, filename, lineno, _step| {
            let mut cmd = Command::new(&world.executable)
                .arg("add")
                .arg(filename)
                .arg(lineno.to_string())
                .stdin(Stdio::piped())
                .spawn()
                .expect("failed to execute spor");

            {
                let stdin = cmd.stdin.as_mut()
                    .expect("Failed to open stdin");
                stdin.write_all("{meta: data}".as_bytes())
                    .expect("Failed to write to stdin");
            }

            let output = cmd.wait_with_output()
                .expect("Failed to read stdout");

            assert_eq!(String::from_utf8_lossy(&output.stdout), "");
        };

        then regex r"^an anchor for (.+) at line (\d+) appears in the listing$" (String, usize) |world, filename, _lineno, _step| {
            let output = Command::new(&world.executable)
                .arg("list")
                .arg(filename)
                .output()
                .expect("failed to execute spor");
            let output = String::from_utf8_lossy(&output.stdout);
            let output: Vec<&str> = output.split("\n").filter(|s| !s.is_empty()).collect();
            assert_eq!(output.len(), 1);

            // TODO: Look for correct output, e.g. it contains filename, has the right line number, etc.
        };

        then "the repository is valid" |world, _step| {
            let status = Command::new(&world.executable)
                .arg("validate")
                .status()
                .expect("failed to execute spor");

            assert!(status.success());
        };

        then "the repository is invalid" |world, _step| {
            let status = Command::new(&world.executable)
                .arg("validate")
                .status()
                .expect("failed to execute spor");

            assert!(!status.success());
        };


    });
}

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |_scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |_scenario| {

});

// A setup function to be called before everything else
fn setup() {}

cucumber! {
    features: "./features", // Path to our feature files
    world: ::World, // The world needs to be the same for steps and the main cucumber call
    steps: &[
        example_steps::steps // the `steps!` macro creates a `steps` function in a module
    ],
    setup: setup, // Optional; called once before everything
    before: &[
        a_before_fn // Optional; called before each scenario
    ],
    after: &[
        an_after_fn // Optional; called after each scenario
    ]
}

// import subprocess

// from radish import given, when, then
