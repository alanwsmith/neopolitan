use anyhow::Result;

fn main() -> Result<()> {
    println!("hello, world");
    Ok(())
}

#[cfg(test)]
mod test {
    use neopolitan::config::Config;
    use neopolitan::site::Site;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    #[test]
    fn solo_build_site() {
        // This is a hack to attempt to built the
        // site before running the rest of the
        // tests. A more appropriate method will
        // be added at a later date.
        let mut site = Site {
            config: Config::default(),
            errors: BTreeMap::new(),
            incompletes: BTreeMap::new(),
            input_root: PathBuf::from("docs-content"),
            output_root: PathBuf::from("docs"),
            pages: BTreeMap::new(),
            pages_dev: BTreeMap::new(),
            files: vec![],
        };
        // These will panic intentionally if
        // they fail for now
        site.load_pages_and_files().unwrap();
        site.copy_files().unwrap();
        site.output_pages().unwrap();
        site.output_errors().unwrap();
        site.output_incompletes().unwrap();
    }
}
