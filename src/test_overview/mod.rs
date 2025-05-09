// TODO: Make this a command line function
// to output the coverage

use crate::helpers::get_file_list;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TestReportCase {
    pub source: String,
    pub source_path: PathBuf,
}

impl TestReportCase {
    pub fn new(source_path: &PathBuf) -> TestReportCase {
        let source = fs::read_to_string(source_path).unwrap();
        TestReportCase {
            source,
            source_path: source_path.clone(),
        }
    }
}

#[derive(Debug)]
pub struct TestReport {
    pub input_root: PathBuf,
    pub output_root: PathBuf,
    pub cases: Vec<TestReportCase>,
}

impl TestReport {
    pub fn new(input_root: &PathBuf, output_root: &PathBuf) -> TestReport {
        let file_list =
            get_file_list(input_root, &vec!["neotest".to_string()]).unwrap();
        let cases = file_list.iter().map(|p| TestReportCase::new(p)).collect();
        TestReport {
            cases,
            input_root: input_root.clone(),
            output_root: output_root.clone(),
        }
    }

    pub fn output_dirs(self) -> Vec<PathBuf> {
        self.cases
            .iter()
            .filter_map(|case| case.source_path.parent())
            .filter_map(|p| p.strip_prefix(&self.input_root).ok())
            .map(|p| self.output_root.clone().join(p))
            .unique()
            .sorted()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solo_make_report() {
        let tr = TestReport::new(
            &PathBuf::from("src"),
            &PathBuf::from("docs-content"),
        );
        dbg!(tr.output_dirs());

        assert!(false);
    }
}
