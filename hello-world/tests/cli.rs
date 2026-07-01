// use std::process::Command;
use assert_cmd::Command;
#[test]
fn works(){
    let mut cmd = Command::cargo_bin("hello-world").unwrap();
    cmd.assert().success();
    // assert!(res);
}

#[test]
fn true_ok(){
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn test_not_ok(){
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
#[test]
fn runs(){
    let mut cmd = Command::cargo_bin("hello-world").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

// To call ->  cargo run --quiet --bin false
// or  cargo run --quiet --bin true

// the true.rs and false.rs after bin means , they are to be run should be seen as output
// when used with std::process::Command
// the new instance of cmd has .output()
// this is Result Type
// .output() is tested with .is_ok() inside assert! macro

//when used assert_cmd
// this has a different type -> command type
// it uses cargo_bin instead of new then .uwrap() it
// this can be tested with cmd.assert().success()

// what it is baffling to me?
// cmd = Command::new("whatever") -> the object is mutable and generates output in the same object
// this is not visible, but accessible with .output()

// assert_cmd 
// Command::cargo_bin("hello-world").unwrap();
// again the output is not visible but cmd.assert() can be used with .success()/ failure()//stdout() etc
// the book has not given these details