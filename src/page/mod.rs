use crate::ast::Ast;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// NOTE: Using source_path here because getting
// the output path is dependent on the system.

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Page<'a> {
    pub source_path: PathBuf,
    pub data: Ast<'a>,
}

