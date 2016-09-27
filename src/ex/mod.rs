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
    pub start: Locator,
    pub end: Option<Locator>,

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
    Quit,
}

#[derive(Debug,PartialEq)]
pub struct Command {
    pub string: String,
    pub selector: Selector,
    pub action: Action,
}
