// #![allow(warnings)]
use crate::source_file::source_file::SourceFile;
use minijinja::context;
use minijinja::Environment;
use minijinja::Source;
use std::env;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn build_site() {
    println!("Making the site");
    fs::create_dir("site").unwrap();

    let mut env = Environment::new();
    env.set_source(Source::from_path(PathBuf::from("./templates")));
    let wrapper = env.get_template("home_page.j2").unwrap();

    let mut source_files: Vec<SourceFile> = vec![];

    // TODO: Remove this when the path is being
    // built in the prod locaiton
    fs::create_dir_all("site/posts/alfa").unwrap();

    env::set_current_dir("content").unwrap();
    for entry in WalkDir::new("./").into_iter() {
        let p = entry.unwrap().path().to_path_buf();
        if let Some(ext) = p.extension() {
            if ext == "neo" {
                let mut sf = SourceFile::new();
                sf.source_path = Some(p.clone());
                sf.source_data = Some(fs::read_to_string(p).unwrap());
                source_files.push(sf);
            }
        }
    }

    env::set_current_dir("../site").unwrap();
    source_files.iter().for_each(|source_file| {
        // dbg!(source_file);
        let output = wrapper
            .render(context!(
                title => source_file.title(),
            ))
            .unwrap()
            .to_string();
        fs::write(source_file.output_path().unwrap(), output).unwrap();
        ()
    });

    env::set_current_dir("..").unwrap();
    let mut source_file = SourceFile::new();
    let lines = vec!["-> title", "", "Alfa Bravo", "", "-> p", "Delta Foxtrot"];
    source_file.source_data = Some(lines.join("\n"));
    let wrapper = env.get_template("home_page.j2").unwrap();
    let output = wrapper
        .render(context!(
            title => source_file.title(),
        ))
        .unwrap()
        .to_string();
    // fs::write("site/index.html", output).unwrap();
    fs::write("site/posts/alfa/index.html", "foxtrot").unwrap();
}
