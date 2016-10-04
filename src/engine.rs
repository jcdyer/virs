use buffer;
use ex;
use std::process;
use std::io::{self, Write};

pub struct Engine {
    pub buffer: buffer::Buffer,
    pub cursor: CursorLocator,
    pub clipboard: String,
}

pub struct CursorLocator {
    line: u64,
    col: u64
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            buffer: buffer::Buffer::new(),
            cursor: CursorLocator::new(),
            clipboard: String::new(),
        }
    }

    pub fn execute(&mut self, command: ex::Command) -> Result<(), String> {
        let range = self.get_selection(&command.selector);
        match command.action {
            ex::Action::Edit(ref filename) => self.execute_edit(range, filename),
            ex::Action::Go => self.execute_go(range),
            ex::Action::Yank => self.execute_yank(range),
            ex::Action::Print => self.execute_print(range),
            ex::Action::Quit => self.execute_quit(),
            _ => self.execute_unknown(command)
        }
    }

    fn get_selection(&self, selector: &ex::Selector) -> (u64, Option<u64>) {
        let start = match selector.start {
            ex::Locator::All => 1,
            ex::Locator::Here => self.cursor.line,
            ex::Locator::Last => self.buffer.content.len() as u64,
            ex::Locator::Line(x) => x,
            ex::Locator::Ahead(offset) => self.cursor.line + offset,
            ex::Locator::Back(offset) => self.cursor.line - offset,
        };
        let end = match selector.end {
            Some(ref location) => match location {
                &ex::Locator::Here => Some(self.cursor.line),
                &ex::Locator::All | &ex::Locator::Last => Some(self.buffer.content.len() as u64),
                &ex::Locator::Line(x) => Some(x),
                &ex::Locator::Ahead(offset) => Some(self.cursor.line + offset),
                &ex::Locator::Back(offset) => Some(self.cursor.line - offset),
            },
            None => match selector.start {
                ex::Locator::All => Some(self.buffer.content.len() as u64),
                _ => None,
            }
        };
        return (start, end)
    }


    fn execute_edit(&mut self, range: (u64, Option<u64>), filename: &str) -> Result<(), String> {
        match buffer::Buffer::open(filename) {
            Ok(buffer) => { 
                self.buffer = buffer;
                self.cursor = CursorLocator::new();
                Ok(())
            },
            Err(_) => Err(format!("Could not open specified file: {}", filename))
        }
    }

    fn execute_go(&mut self, range: (u64, Option<u64>)) -> Result<(), String> {
        let line = match range.1 {
            Some(x) => x,
            None => range.0,
        };
        self.cursor = CursorLocator { line: line, col: 1 };
        Ok(())
    }

    fn execute_yank(&mut self, range: (u64, Option<u64>)) -> Result<(), String> {
        let mut yanked = String::new();
        let end = match range.1 {
            Some(x) => x,
            None => range.0,
        };
        for line in range.0 .. (end + 1) {
            let offset = (line - 1) as usize;
            yanked.push_str(&self.buffer.content[offset]);
            yanked.push('\n');
        }
        self.clipboard = yanked;
        Ok(())
    }

    fn execute_print(&self, range: (u64, Option<u64>)) -> Result<(), String> {
        let end = match range.1 {
            Some(x) => x,
            None => range.0,
        };
        for line in range.0 .. (end + 1) {
            let offset = (line - 1) as usize;
            let output = format!("{} {}\n", line, &self.buffer.content[offset]);
            io::stdout().write(output.as_bytes()).ok();
        };
        Ok(())
    }

    fn execute_quit(&self) -> Result<(), String> {
        process::exit(0);
    }

    fn execute_unknown(&self, command: ex::Command) -> Result<(), String> {
        Err(format!("Unknown command {:?}", command))
    }
}

impl CursorLocator {
    pub fn new() -> Self {
        CursorLocator { line: 1, col: 1 }
    }
}

#[cfg(test)]
mod tests {
    use ex;
    use super::*;

    #[test]
    fn execute_yank() {
        let mut engine = Engine::new();
        engine.buffer.content.push("First line.".to_string());
        let cmd = ex::Command {
            string: "1y".to_string(),
            action: ex::Action::Yank,
            selector: ex::Selector { start: ex::Locator::Line(1), end: None },
        };
        match engine.execute(cmd) {
            Ok(_) => assert_eq!(&engine.clipboard, "First line.\n"),
            Err(e) => panic!(e),
        };
    }
}
