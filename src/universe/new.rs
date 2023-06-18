use crate::universe::universe::Universe;
use std::collections::HashMap;

impl Universe<'_> {
    pub fn new() -> Universe<'static> {
        Universe {
            assets_dir: None,
            content_dir: None,
            content_files: HashMap::new(),
            env: None,
            output_root: None,
        }
    }
}
