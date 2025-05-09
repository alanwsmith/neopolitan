#![allow(unused)]
// TODO: Make this a command line function
// to output the coverage

use crate::helpers::*;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use itertools::Itertools;
use minijinja::{Environment, context};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TestOverviewCase {
    pub source: String,
    pub source_path: PathBuf,
}

impl TestOverviewCase {
    pub fn new(source_path: &PathBuf) -> TestOverviewCase {
        let source = fs::read_to_string(source_path).unwrap();
        TestOverviewCase {
            source,
            source_path: source_path.clone(),
        }
    }
}
