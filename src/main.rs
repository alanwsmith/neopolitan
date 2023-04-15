#![allow(warnings)]
use minijinja::context;
use minijinja::Environment;
use minijinja::Source;
use neopolitan::parse::parse;
use neopolitan::wrapper::wrapper::*;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;
use walkdir::{DirEntry, Error, WalkDir};

fn main() {
    // let env = create_env("./templates");
    //

    do_copy("./content", "./site").unwrap();

    // for entry in WalkDir::new("./content").into_iter().filter_map(|e| e.ok()) {
    //     println!("{:?}", entry.path().canonicalize());
    // }

    //let paths = fs::read_dir("./content").unwrap();
    //for path in paths {
    //    let file_path = path.unwrap().path();
    //    // dbg!(file_path);
    //    // if file_path.find(".neo") != None {
    //    //  println!("Path: {}", file_path)
    //    //}
    //}

    // let source = fs::read_to_string("./content/index.neo").unwrap();
    // let payload = parse(source.as_str()).unwrap().1;
    // let output = render_template(payload, env, "main.html");
    // fs::write("./site/index.html", output).unwrap();

    dbg!("done");
}

fn create_env(path: &str) -> Environment<'static> {
    let mut env = Environment::new();
    env.set_source(Source::from_path(path));
    env
}

fn render_template(payload: Wrapper, env: Environment, template: &str) -> String {
    let tmpl = env.get_template(template).unwrap();
    tmpl.render(context!(payload => &payload))
        .unwrap()
        .to_string()
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn do_copy(source_dir: &str, dest_dir: &str) -> Result<(), Error> {
    let walker = WalkDir::new(source_dir).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let p = entry?.path().to_path_buf();
        let sub_path = &p.strip_prefix(source_dir);
        let mut dest_path = PathBuf::from(dest_dir);
        dest_path.push(sub_path.as_ref().unwrap());
        if p.is_dir() {
            if !dest_path.exists() {
                create_dir_all(dest_path).unwrap();
            }
        } else {
            // copy or write the parsed file
            if p.extension().unwrap() == "neo" {
                dbg!((p.display().to_string(), dest_path.display().to_string()));
            } else {
                fs::copy(p, dest_path);
                // dbg!((p.display().to_string(), dest_path.display().to_string()));
            }
        }
    }
    Ok(())
}
