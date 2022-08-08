use std::process::Command; //Run programs
use assert_cmd::prelude::*; //add methods on commands
use predicates::prelude::*; //Used for writing assertions

#[test]
fn run_with_defaults() -> Result<(),Box<dyn std::error::Error>>{
    Command::cargo_bin("dogsay")
        .expect("binary exist")
        .assert()
        .success()
        .stdout(predicate::str::contains("Woof!"));

    Ok(())
}

fn fail_on_non_existing_file() -> Result<(), Box<std::error::Error>>{
    Command::cargo_bin("dogsay")
        .expect("binary exist")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure();
    Ok(())
}