use nom;
use nom::{IResult, ErrorKind, digit, eof};
use nom::IResult::{Done, Error};
use super::{Command, Selector, Locator, Action};
use self::utils::*;

mod utils {
    use nom;
    use nom::{IResult, ErrorKind};
    use nom::IResult::{Done, Incomplete, Error};

    pub fn tag_str<'a>(input: &'a str, tag: &'static str) -> IResult<&'a str, &'a str> {
        let taglen = tag.len();
        if input.len() < taglen {
            Error(nom::Err::Position(ErrorKind::Tag, input))
        } else {
            let (prefix, remainder) = input.split_at(taglen);
            if prefix == tag {
                Done(remainder, prefix)
            } else {
                Error(nom::Err::Position(ErrorKind::Tag, input))
            }
        }
    }

    pub fn map_result<I, O, F, N, E>(ires: IResult<I, O>, f: F) -> IResult<I, N>
        where F : Fn(O) -> Result<N, E> {
        match ires {
            Done(input, output) => {
                match f(output) {
                    Ok(result) => Done(input, result),
                    Err(_) => Error(
                        nom::Err::Position(ErrorKind::Digit, input)
                    ),
                }
            },
            Error(e) => Error(e),
            Incomplete(i) => Incomplete(i),
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use nom;
        use nom::{ErrorKind};
        use nom::IResult::{Done, Error};

        #[test]
        fn match_tag_str() {
            assert_eq!(tag_str("abc", "a"), Done("bc", "a"));
            assert_eq!(tag_str("abc", "ab"), Done("c", "ab"));
            assert_eq!(tag_str("abc", "abc"), Done("", "abc"));
            assert_eq!(tag_str("abc", ""), Done("abc", ""));
        }

        #[test]
        fn nomatch_tag_str() {
            assert_eq!(tag_str("abc", "b"), Error(nom::Err::Position(ErrorKind::Tag, "abc")));
            assert_eq!(tag_str("abc", "abe"), Error(nom::Err::Position(ErrorKind::Tag, "abc")));
            assert_eq!(tag_str("abcd", "abcderian"), Error(nom::Err::Position(ErrorKind::Tag, "abcd")));
        }

        #[test]
        fn tag_str_against_empty() {
            assert_eq!(tag_str("", "a"), Error(nom::Err::Position(ErrorKind::Tag, "")));
        }
    }
}

pub fn parse_command<'a>(input: &'a str) -> IResult<&str, Command> {
    tuple!(input, parse_selector , parse_action, eof).map(|(selector, action, _)| {
        Command { string: input.to_string(), selector: selector, action: action }
    })
}

fn parse_selector(input: &str) -> IResult<&str, Selector> {
    let (input, start) = parse_locator(input).unwrap();
    let (input, optend) = if input.chars().nth(0) == Some(',') {
        let (input, end) = parse_locator(input.split_at(1).1).unwrap();
        (input, Some(end))
    } else {
        (input, None)
    };
    Done(input, Selector { start: start, end: optend })
}

fn action_quit(input: &str) -> IResult<&str, Action> {
    tag_str(input, "q").map(|_| { Action::Quit })
}
fn action_yank(input: &str) -> IResult<&str, Action> {
    tag_str(input, "y").map(|_| { Action::Yank })
}
fn action_print(input: &str) -> IResult<&str, Action> {
    tag_str(input, "p").map(|_| { Action::Print })
}
fn action_put(input: &str) -> IResult<&str, Action> {
    tag_str(input, "put").map(|_| { Action::Put })
}
fn action_delete(input: &str) -> IResult<&str, Action> {
    tag_str(input, "d").map(|_| { Action::Delete })
}
fn action_append(input: &str) -> IResult<&str, Action> {
    tag_str(input, "a").map(|_| { Action::Append })
}
fn action_edit(input: &str) -> IResult<&str, Action> {
    match tag_str(input, "e") {
        Done(input, _) => parse_filename(input).map(|filename| { Action::Edit(filename) }),
        IResult::Incomplete(x) => IResult::Incomplete(x),
        Error(x) => IResult::Error(x),
    }
}
fn action_write(input: &str) -> IResult<&str, Action> {
    match tag_str(input, "w") {
        Done(input, _) => parse_filename(input).map(|filename| { Action::Write(filename) }),
        IResult::Incomplete(x) => IResult::Incomplete(x),
        Error(x) => IResult::Error(x),
    }
}
fn action_go(input: &str) -> IResult<&str, Action> {
    eof(input).map(|_| { Action::Go })
}

fn action_unknown(input: &str) -> IResult<&str, Action> {
    Error(nom::Err::Position(ErrorKind::Tag, input))
}

fn parse_action(input: &str) -> IResult<&str, Action> {
    alt!(input, action_quit|action_yank|action_put|action_print|action_delete|action_append|action_edit|action_go|action_write|action_unknown)
}

fn parse_locator(input: &str) -> IResult<&str, Locator> {
    match input.chars().nth(0) {
        Some('.') => Done(input.split_at(1).1, Locator::Here),
        Some('%') => Done(input.split_at(1).1, Locator::All),
        Some('$') => Done(input.split_at(1).1, Locator::Last),
        Some('+') => {
            parse_u64(input.split_at(1).1).map(|distance| {
                Locator::Ahead(distance)
            })
        },
        Some('-') => {
            parse_u64(input.split_at(1).1).map(|distance| {
                Locator::Back(distance)
            })
        },
        Some('0' ... '9') => {
            parse_u64(input).map(|lineno| {
                Locator::Line(lineno)
            })
        },
        _ => Done(input, Locator::Here),
    }
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
   // TODO: Handle too-large integers
   map_result(digit(input), |o|{o.parse()})
}

fn parse_filename<'a>(input: &'a str) -> IResult<&'a str, String> {
    let mut input = input;
    while input.chars().nth(0) == Some(' ') {
        input = input.split_at(1).1;
    }
    Done("", input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{Command, Selector, Locator, Action};
    use nom::IResult::Done;

    fn assert_command_equal(cmd_string: &str, selector: Selector, action: Action) {
        let cmd = parse_command(cmd_string);
        let expected_result = Command {
            string: cmd_string.to_string(),
            selector: selector,
            action: action,
        };
        assert_eq!(cmd, Done("", expected_result));
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
    fn line_go() {
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

    // TODO: Handle this failure gracefully.
    #[test]
    #[should_panic(expected="unwrap() called on an IResult that is Error")]
    fn too_large_line() {
        assert_command_equal(
            "999999999999999999999999999999999999y",
            Selector {
                start: Locator::Line(3),
                end: None,
            },
            Action::Yank,
        );
    }
}
