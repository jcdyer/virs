use std::io;
use std::io::Write;
use ex::parser;

pub mod ex;
pub mod buffer;

fn main() {
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
    }
}
