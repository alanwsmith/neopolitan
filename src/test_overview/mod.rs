#![allow(unused)]
// TODO: Make this a command line function
// to output the coverage

use crate::helpers::*;
use crate::test_overview_case::TestOverviewCase;
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
pub struct TestOverview<'a> {
    pub env: Environment<'a>,
    pub input_root: PathBuf,
    pub output_root: PathBuf,
    pub cases: Vec<TestOverviewCase>,
}

impl TestOverview<'_> {
    // pub fn make_index_files(&self) -> Result<()> {
    //     let
    //     // let outputs: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
    //     // for dir in &self.output_dirs() {
    //     //     let child_dirs = get_dirs_in_dir(dir)?;
    //     //     dbg!(child_dirs);
    //     // }
    //     Ok(())
    // }

    pub fn make_output_dirs(&self) -> Result<()> {
        for output_dir in &self.output_dirs() {
            std::fs::create_dir_all(output_dir)?;
        }
        Ok(())
    }

    pub fn new<'a>(
        input_root: &'a PathBuf,
        output_root: &'a PathBuf,
    ) -> TestOverview<'a> {
        let env = Environment::new();
        let file_list =
            get_file_list(input_root, &vec!["neotest".to_string()]).unwrap();
        let cases =
            file_list.iter().map(|p| TestOverviewCase::new(p)).collect();
        TestOverview {
            cases,
            env,
            input_root: input_root.clone(),
            output_root: output_root.clone(),
        }
    }

    pub fn output_dirs(&self) -> Vec<PathBuf> {
        self.cases
            .iter()
            .filter_map(|case| case.source_path.parent())
            .filter_map(|p| p.strip_prefix(&self.input_root).ok())
            .map(|p| self.output_root.clone().join(p))
            .unique()
            .sorted()
            .collect()
    }

    pub fn output_index_files(&self, working_dir: &PathBuf) -> Result<()> {
        let child_dirs = get_dirs_in_dir(&working_dir)?;
        dbg!(child_dirs);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solo_make_report() {
        let input_root = PathBuf::from("src");
        let output_root = PathBuf::from("docs-content/_test-overview");
        let tr = TestOverview::new(&input_root, &output_root);
        tr.make_output_dirs();
        let _ = tr.output_index_files(&tr.output_root);
        assert!(false);
    }
}
