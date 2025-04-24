use crate::section_category::SectionCategory;

pub struct NeoConfig {
    pub section_kinds: Vec<(String, SectionCategory)>,
}

impl NeoConfig {
    pub fn default() -> NeoConfig {
        NeoConfig {
            section_kinds: vec![],
        }
    }
}
