#![allow(warnings)]
pub struct Builder {
    pub source: String,
}

impl Builder {
    pub fn new(source: String) -> Builder {
        Builder { source }
    }
}
