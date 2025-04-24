use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum SectionBound {
    End,
    Full,
    Start,
}
