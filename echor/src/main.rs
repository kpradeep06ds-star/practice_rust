//use clap::Command;
// to get more programs from clap
// use {}
use clap::{Arg, ArgAction, Command};

fn main() {
    // println!("Hello, world!");
    // println!("{:?}", std::env::args()); // without clap

    // let _matches = Command::new("echor").version(
    //     "0.1.0"
    // ).author(
    //     "PK"
    // ).about(
    //     "rust echo"
    // ).get_matches(

    // );

    // with Arg and ArgAction
    let matches = Command::new("echor").version(
        "0.1.0"
    ).author(
        "PK"
    ).about(
        "Rust"
    ).arg(
        Arg::new("text").value_name( // this is probably creating a new text positional parameter
            "TEXT" // the default value seen on screen as TEXT
        ).help(
            "Input text"
        ).required(true).num_args(
            1.. // this paramter is always required - sort of mandatory , not sure why 1.., book didn't bother abuot explaining
        ),
    ).arg(
        Arg::new("omit_newline").short(
            'n' // paramter after text
        ).action(
            ArgAction::SetTrue // not sure what it does and why suddently a PASCAL CASE?? WTF wrong with people
        ).help("Don't print newline")
    ).get_matches();

    println!("{:#?}", matches);

}
// std::env::args -> 
// std::process -> to handle external proces (like ls etc)
// to circumvent to enable to read the current process -> we then used assert_cmd
// in here std::env is different -> it captures / or interact with environments
// this doesn't work: std::env::args(); with "{}" -> but works with "{:?}"
// usage of clap -> command line argument parser is better toolkit
