#[macro_use]
extern crate nom;

use std::io;
use std::io::Write;
use ex::parser;
use nom::IResult::{Done, Incomplete, Error};

pub mod engine;
pub mod ex;
pub mod buffer;

fn main() {
    let mut engine = engine::Engine::new();
    engine.buffer = match buffer::Buffer::open("src/main.rs") {
        Ok(buffer) => buffer,
        Err(error) => panic!(error),
    };
    loop {
        match io::stdout().write(":".as_bytes()) {
            Ok(_) => {},
            Err(_) => continue,
        };
        match io::stdout().flush() {
            Ok(_) => {},
            Err(_) => continue,
        };
        let mut command_string = String::new();
        match io::stdin().read_line(&mut command_string) {
            Ok(_) => {},
            Err(_) => continue,
        };
        match parser::parse_command(command_string.trim()) {
            Done("", command) => {
                println!("{:?}", command);
                match engine.execute(command) {
                    Ok(()) => {},
                    Err(string) => println!("{}", string),
                }
            },
            Done(extra, command) => {
                println!("Invalid command: {}. Extra characters found at the end: {}", command.string, extra)
            },
            Error(err) => println!("{:?}", err),
            Incomplete(_) => panic!("Should not receive incomplete"),
        }

    }
}
