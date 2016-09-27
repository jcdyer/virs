use super::{Command, Selector, Locator, Action};

pub fn parse_command<'a>(input: &'a str) -> Command {
    let command_str = input.to_string();

    let (selector, input) = parse_selector(input);
    let (action, input) = parse_action(input);
    if input != "" {
        println!("Extra input: {:?}", input);
    };
    Command {
        string: command_str,
        selector: selector,
        action: action
    }
}

fn parse_selector<'a>(input: &'a str) -> (Selector, &'a str) {
    let (start, input) = parse_locator(input);
    let (optend, input) = if input.chars().nth(0) == Some(',') {
        let (end, input) = parse_locator(input.split_at(1).1);
        (Some(end), input)
    } else {
        (None, input)
    };
    (Selector { start: start, end: optend }, input)
}

fn parse_locator<'a>(input: &'a str) -> (Locator, &'a str) {
    match input.chars().nth(0) {
        Some('.') => (Locator::Here, input.split_at(1).1),
        Some('%') => (Locator::All, input.split_at(1).1),
        Some('$') => (Locator::Last, input.split_at(1).1),
        Some('+') => {
            let (distance, input) = parse_integer(input.split_at(1).1);
            (Locator::Ahead(distance), input)
        },
        Some('-') => {
            let (distance, input) = parse_integer(input.split_at(1).1);
            (Locator::Back(distance), input)
        },
        Some('0' ... '9') => {
            let (lineno, input) = parse_integer(input);
            (Locator::Line(lineno), input)
        },
        _ => (Locator::Here, input),
    }
}

fn parse_integer<'a>(input: &'a str) -> (u64, &'a str) {
    let mut input: &str = input;
    let mut buf = String::with_capacity(8);
    loop {
        // TODO: This is broken.
        match input.chars().nth(0) {
            Some(x) => match x {
                '0' ... '9' => {
                    buf.push(x);
                    input = input.split_at(1).1;
                },
                _ => break,
            },
            None => break,
        };
    };
    let i = u64::from_str_radix(&buf, 10).ok().unwrap();
    (i, input)
}

fn parse_action<'a>(input: &'a str) -> (Action, &'a str) {
    match input.chars().nth(0) {
        None => (Action::Go, input),
        Some('q') => (Action::Quit, input.split_at(1).1),
        Some('y') => (Action::Yank, input.split_at(1).1),
        Some('p') => (Action::Print, input.split_at(1).1),
        Some('d') => (Action::Delete, input.split_at(1).1),
        Some('a') => (Action::Append, input.split_at(1).1),
        Some('e') => {
            let (filename, input) = parse_filename(input.split_at(1).1);
            (Action::Edit(filename), input)
        },
        Some('g') => {
            let (inner_action, input) = parse_action(input.split_at(1).1);
            (Action::Global(Box::new(inner_action)), input)
        },
        Some(_) => panic!("Unknown action"),
    }
}

fn parse_filename<'a>(input: &'a str) -> (String, &'a str) {
    let mut input = input;
    while input.chars().nth(0) == Some(' ') {
        input = input.split_at(1).1;
    }
    (input.to_string(), "")
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{Command, Selector, Locator, Action};

    fn assert_command_equal(cmd_string: &str, selector: Selector, action: Action) {
        let cmd = parse_command(cmd_string);
        let expected_result = Command {
            string: cmd_string.to_string(),
            selector: selector,
            action: action,
        };
        assert_eq!(cmd, expected_result);
    }

    #[test]
    fn here_print() {
        assert_command_equal(".p", Selector {start: Locator::Here, end: None}, Action::Print);
    }

    #[test]
    fn all_delete() {
        assert_command_equal("%d", Selector {start: Locator::All, end: None}, Action::Delete);
    }

    #[test]
    fn lineno_go() {
        assert_command_equal("14", Selector {start: Locator::Line(14), end: None}, Action::Go);
    }

    #[test]
    fn simple_range_delete() {
        assert_command_equal(
            "3,4444d",
            Selector {start: Locator::Line(3), end: Some(Locator::Line(4444))},
            Action::Delete,
        );
    }

    #[test]
    fn relative_range() {
        assert_command_equal(
            "-3,+0y",
            Selector {start: Locator::Back(3), end: Some(Locator::Ahead(0))},
            Action::Yank,
        );
    }
}
