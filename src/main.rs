use std::io;
use std::io::Write;
use ex::parser;

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
        let command = parser::parse_command(command_string.trim());
        println!("{:?}", command);
        match engine.execute(command) {
            Ok(()) => {},
            Err(string) => println!("{}", string),
        }
    }
}
