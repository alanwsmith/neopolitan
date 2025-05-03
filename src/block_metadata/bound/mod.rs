use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BlockBound {
    End,
    Full,
    Start,
}
