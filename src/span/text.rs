use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TextSpan {
    pub kind: String,
    pub text: String,
}
