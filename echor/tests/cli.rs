use std::fs::{self, read_to_string};

use assert_cmd::Command;
use predicates::prelude::*;
use anyhow::Result;

#[test]
fn dies_no_args(){
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert().failure().stderr(predicate::str::contains("Usage"));
}

#[test]
fn runs(){
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}

#[test]
fn hello1(){
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}

#[test]
fn hello2() -> Result<()> {
    let expected = fs::read_to_string("tests/expected/hello2.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello", "there"]).assert().success().stdout(expected);

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> Result<()>{
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin("echor")?.args(args).output().expect("fail");

    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8");
    assert_eq!(stdout, expected);
    Ok(())
}

#[test]
fn hello3() -> Result<()>{
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello4() -> Result<()>{
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello5() -> Result<()>{
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello6() -> Result<()>{
    run(&["-n","Hello",  "there"], "tests/expected/hello2.n.txt")
}
