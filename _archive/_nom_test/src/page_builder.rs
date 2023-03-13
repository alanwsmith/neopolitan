#![allow(warnings)]
#[derive(Debug)]
pub struct PageBuilder {
    pub input: Option<String>,
}

impl PageBuilder {
    pub fn new() -> PageBuilder {
        PageBuilder { input: None }
    }
}
