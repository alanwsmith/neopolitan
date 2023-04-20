#![allow(warnings)]
use minijinja::Environment;

pub struct Universe<'a> {
    pub env: Option<Environment<'a>>,
}

impl Universe<'_> {
    pub fn new() -> Universe<'static> {
        Universe { env: None }
    }
}
