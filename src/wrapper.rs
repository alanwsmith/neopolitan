use crate::section::Section;

#[derive(Debug, PartialEq)]
pub enum Wrapper {
    Page { children: Option<Vec<Section>> },
}
