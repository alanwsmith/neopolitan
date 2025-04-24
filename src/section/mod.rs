use crate::section_category::SectionCategory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Section {
    pub category: SectionCategory,
    pub kind: String,
}
