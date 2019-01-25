extern crate cucumber_rust;

use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

use super::super::world::World;

steps!(World => {

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
            let output = Command::new(&world.executable)
                .arg("status")
                .output()
                .expect("failed to execute spor");
            let output = String::from_utf8_lossy(&output.stdout);
            let output: Vec<&str> = output.split("\n").filter(|s| !s.is_empty()).collect();
            assert!(output.is_empty());
        };

        then "the repository is invalid" |world, _step| {
            let output = Command::new(&world.executable)
                .arg("status")
                .output()
                .expect("failed to execute spor");
            let output = String::from_utf8_lossy(&output.stdout);
            let output: Vec<&str> = output.split("\n").filter(|s| !s.is_empty()).collect();
            assert!(!output.is_empty());
        };


    });
