use crate::page_ast::PageAst;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// NOTE: Using source_path here because getting
// the output path is dependent on the system.

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Page {
    pub path: PathBuf,
    pub data: PageAst,
}
