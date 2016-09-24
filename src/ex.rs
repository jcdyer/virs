#[derive(Debug)]
enum Locator {
    Last,
    Here,
    All,
    Ahead(u64),
    Back(u64),
    Line(u64),
}

#[derive(Debug)]
struct Selector {
    start: Locator,
    end: Option<Locator>,

}

#[derive(Debug)]
enum Action {
    Yank,
    Delete,
    Global(Box<Action>),
    Go,
    Print,
}

#[derive(Debug)]
pub struct Command {
    string: String,
    selector: Selector,
    action: Action,
}

pub fn parse_command<'a>(input: &'a str) -> Command {
    let command_str = input.to_string();
    
    let (range, input) = parse_range(input);
    let (action, input) = parse_action(input);
    if input != "" {
        println!("Extra input: {:?}", input);
    };
    Command {
        string: command_str,
        selector: range,
        action: action
    }
}
    
fn parse_range<'a>(input: &'a str) -> (Selector, &'a str) {
    let (start, input) = parse_locator(input);
    (Selector { start: start, end: None }, input)
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
        Some('y') => (Action::Yank, input.split_at(1).1),
        Some('p') => (Action::Print, input.split_at(1).1),
        Some('d') => (Action::Delete, input.split_at(1).1),
        Some('g') => {
            let (inner_action, input) = parse_action(input.split_at(1).1);
            (Action::Global(Box::new(inner_action)), input)
        },
        Some(_) => panic!("Unknown action"),
        None => (Action::Go, input),
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn test_parse_here_print() {
        let command = parse_command(".p");
        let expected_result = Command {
            string: ".p".to_string(),
            selector: Selector { start: Locator::Here, end: None },
            action: Action::Print,
        };

        assert_eq!(command, expected_result);
    }
}
