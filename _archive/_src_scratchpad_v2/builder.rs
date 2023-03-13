#![allow(warnings)]
pub struct Builder {
    pub source: Option<String>,
}

impl Builder {
    pub fn new(source: String) -> Builder {
        Builder {
            source: Some(source),
        }
    }
}
