pub mod parser;

#[derive(Debug,PartialEq)]
pub enum Locator {
    Last,
    Here,
    All,
    Ahead(u64),
    Back(u64),
    Line(u64),
}

#[derive(Debug,PartialEq)]
pub struct Selector {
    start: Locator,
    end: Option<Locator>,

}

#[derive(Debug,PartialEq)]
pub enum Action {
    Yank,
    Delete,
    Global(Box<Action>),
    Edit(String),
    Go,
    Print,
    Append,
}

#[derive(Debug,PartialEq)]
pub struct Command {
    string: String,
    selector: Selector,
    action: Action,
}
