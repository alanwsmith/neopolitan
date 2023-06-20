// #![allow(warnings)]
use minijinja::context;
use minijinja::Environment;
use minijinja::Source;
use std::path::PathBuf;
// use crate::source_file::source_file::SourceFile;
// use crate::files::files::Files;
// use std::fs;
//use walkdir::Error;
// use walkdir::WalkDir;

use std::fs;

pub fn build_site() {
    println!("Making the site");
    fs::create_dir("site").unwrap();
    let mut env = Environment::new();
    env.set_source(Source::from_path(PathBuf::from("./templates")));
    let wrapper = env.get_template("home_page.j2").unwrap();
    let output = wrapper
        .render(context!(
            title => "ALFA BRAVO CHARLIE",
        ))
        .unwrap()
        .to_string();
    fs::write("site/index.html", output).unwrap();
}
