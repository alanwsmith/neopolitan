// TODO: Make this a command line function
// to output the coverage

use crate::helpers::*;
use crate::test_overview_case::TestOverviewCase;
use anyhow::Result;
use itertools::Itertools;
use minijinja::syntax::SyntaxConfig;
use minijinja::{Environment, Value, context};
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
    pub fn load_templates(&mut self) -> Result<()> {
        self.env
            .add_template(
                "index",
                r#"-- title

Test Overview

-- html

<div><a href="..">Up</a></div>
<ul>
    [!- for link in links -!]
        <li>
        <a href="[@ link @]/">[@ link @]</a>
        </li>
    [!- endfor -!]
</ul>
"#,
            )
            .unwrap();

        Ok(())
    }

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
        let mut env = Environment::new();
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
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

    pub fn output_index_files(&mut self, working_dir: &PathBuf) -> Result<()> {
        let file_output = working_dir.join("index.neo");
        dbg!(&file_output);
        let child_dirs: Vec<PathBuf> = get_dirs_in_dir(&working_dir)?
            .iter()
            .filter_map(|p| p.strip_prefix(&working_dir).ok())
            .map(|p| p.to_path_buf())
            .sorted()
            .collect();
        let links = Value::from_serialize(&child_dirs);
        let template = self.env.get_template("index")?;
        let output = template.render(context![links])?;
        fs::write(file_output, output)?;
        for child_dir in &child_dirs {
            dbg!(&child_dir);
            let next_dir = working_dir.join(&child_dir);
            dbg!(&next_dir);
            self.output_index_files(&next_dir)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[test]
    fn solo_make_report() -> Result<()> {
        let input_root = PathBuf::from("src");
        let output_root = PathBuf::from("docs-content/_test-overview");
        let mut tr = TestOverview::new(&input_root, &output_root);
        tr.load_templates()?;
        tr.make_output_dirs()?;
        tr.output_index_files(&tr.output_root.clone())?;
        // assert!(false);
        Ok(())
    }
}
