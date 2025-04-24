use crate::{section::Section, section_bound::SectionBound};

pub enum SectionCategory {
    Basic {
        attrs: Vec<String>,
        bound: SectionBound,
        chidren: Vec<Section>,
        end_section: Option<Box<Section>>,
        flags: Vec<String>,
        source_head: String,
        source_body: Option<String>,
    },
    Block {
        spans: Vec<String>,
    },
    CheckListItem,
    CheckList,
    CSS,
    CSV,
    End,
    Html,
    JavaScript,
    Json5,
    List,
    ListItem,
    Olist,
    OlistItem,
    Raw,
}
