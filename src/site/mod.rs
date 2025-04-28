use crate::section::Section;
use std::collections::BTreeMap;
use std::path::PathBuf;

pub struct Site {
    input_root: Option<PathBuf>,
    output_root: Option<PathBuf>,
    pages: BTreeMap<PathBuf, Vec<Section>>,
}
