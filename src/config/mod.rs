use crate::section_category::SectionCategory;

pub struct Config {
    pub section_kinds: Vec<(String, SectionCategory)>,
}

impl Config {
    pub fn default() -> Config {
        Config {
            section_kinds: vec![],
        }
    }
}
