use rustbox;
use rustbox::{RustBox, InitError, Event, Key, Color};

use super::buffer::Buffer;
use super::engine::Engine;

pub struct IO {
    pub rustbox: RustBox,
}

impl IO {
    pub fn new() -> Result<Self, InitError> {
        match RustBox::init(Default::default()) {
            Ok(rustbox) => Ok(IO { rustbox: rustbox, }),
            Err(err) => Err(err),
        }
    }

    pub fn readline(&self) -> String {
        let mut input = String::with_capacity(16);
        loop {
            match self.rustbox.poll_event(false) {
                Ok(Event::KeyEvent(key)) => {
                    match key {
                        Key::Char('\n') | Key::Enter => break,
                        Key::Char(c) => {
                            input.push(c);
                            self.rustbox.print_char(
                                input.len() + 1,
                                self.status_line(),
                                rustbox::RB_BOLD,
                                Color::White,
                                Color::Black,
                                c
                            );
                            self.rustbox.set_cursor((input.len() + 2) as isize, self.status_line() as isize);
                            self.rustbox.present()

                            //self.set_status(&format!(":{}", input));
                        },
                        Key::Backspace => {
                            match input.pop() {
                                Some(_) => {

                                    self.rustbox.print_char(
                                        input.len() + 2,
                                        self.status_line(),
                                        rustbox::RB_BOLD,
                                        Color::White,
                                        Color::Black,
                                        ' '
                                    );
                                    self.rustbox.set_cursor((input.len() + 2) as isize, self.status_line() as isize);
                                    self.rustbox.present()
                                },
                                None => {},
                            }
                        },
                        Key::Esc => {
                            input = "".to_string();
                            break;
                        },
                        _ => break,
                    }
                },
                _ => {},
            };
        }
        input
    }
    pub fn show_buffer(&self, buffer: &Buffer, engine: &Engine) {
        for i in 0 .. self.rustbox.height() - 3 {
            let line = buffer.top_visible + i;
            self.rustbox.print(1, i+1, rustbox::RB_BOLD, Color::White, Color::Black, &buffer.content[line]);
        }
        self.rustbox.set_cursor(engine.cursor.col as isize, engine.cursor.line as isize);
        self.rustbox.present();
    }

    pub fn set_status(&self, status: &str) {
        self.rustbox.clear();
        self.rustbox.print(1, self.status_line(), rustbox::RB_BOLD, Color::White, Color::Black, status);
        self.rustbox.present();
    }

    pub fn status_line(&self) -> usize {
        self.rustbox.height() - 1
    }
}
