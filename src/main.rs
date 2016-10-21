extern crate rustbox;

#[macro_use]
extern crate nom;

use std::thread;
use std::time;

use nom::IResult::{Done, Incomplete, Error};

use ex::{parser};

pub mod display;
pub mod engine;
pub mod ex;
pub mod buffer;

fn sleep(n: u64) {
    thread::sleep(time::Duration::from_secs(n));
}

fn main() {
    let mut io = match display::IO::new() {
        Ok(io) => io,
        Err(err) => panic!("{}", err),
    };
    let mut engine = engine::Engine::new(&mut io);
    engine.buffer = match buffer::Buffer::open("src/main.rs") {
        Ok(buffer) => buffer,
        Err(error) => panic!(error),
    };
    loop {
        let input = match engine.io.rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(rustbox::Key::Char(':'))) => {
                engine.io.set_status(":");
                engine.io.rustbox.set_cursor(2, engine.io.status_line() as isize);
                engine.io.rustbox.present();
                engine.mode = engine::Mode::Ex;
                Some(engine.io.readline())
            },
            Ok(rustbox::Event::KeyEvent(rustbox::Key::Char('q'))) => break,
            Ok(rustbox::Event::KeyEvent(rustbox::Key::Char(x))) => {
                engine.io.set_status(
                    &format!("{}: Only ex mode implemented. Press ':' to enter commands or 'q' to quit", x)
                );
                None
            },
            Ok(_) => {
                engine.io.set_status("Only ex mode implemented. Press ':' to enter commands or 'q' to quit");
                None
            },
            _ => continue,
        };
        match input {
            Some(command_string) => {
                match parser::parse_command(&command_string) {
                    Done("", command) => {
                        match engine.execute(&command) {
                            Ok(continuable) => if continuable {
                                continue;
                            } else {
                                engine.io.set_status(&format!("Received exit command: {:?}", command));
                                sleep(3);
                                break;
                            },
                            Err(string) => engine.io.set_status(&format!("{}", string)),
                        }
                    },
                    Done(extra, command) => engine.io.set_status(
                        &format!(
                            "Invalid command: {}. Extra characters found at the end: {}",
                            command.string,
                            extra
                        )
                    ),
                    Error(err) => engine.io.set_status(&format!("Error: {:?}", err)),
                    Incomplete(_) => panic!("Should not receive incomplete"),
                };
                engine.mode = engine::Mode::Normal;
            },
            None => {
                continue
            }
        }
    }
}
