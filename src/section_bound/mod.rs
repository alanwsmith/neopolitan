use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum SectionBound {
    End,
    Full,
    Start,
}
