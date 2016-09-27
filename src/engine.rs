use buffer;
use ex;
use std::process;

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
            cursor: CursorLocator { line: 1, col: 1 },
            clipboard: String::new(),
        }
    }

    pub fn execute(&mut self, command: ex::Command) -> Result<(), &'static str> {
        let range = self.get_selection(&command.selector);
        match command.action {
            ex::Action::Edit(_) => self.execute_edit(range, command.action),
            ex::Action::Yank => self.execute_yank(range, command.action),
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


    fn execute_edit(&mut self, range: (u64, Option<u64>), action: ex::Action) -> Result<(), &'static str> {
        Ok(())
    }

    fn execute_yank(&mut self, range: (u64, Option<u64>), action: ex::Action) -> Result<(), &'static str> {
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

    fn execute_quit(&self) -> Result<(), &'static str> {
        process::exit(0);
    }

    fn execute_unknown(&self, command: ex::Command) -> Result<(), &'static str> {
        Err("Unknown command")
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
