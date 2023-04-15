#![allow(warnings)]
use minijinja::context;
use minijinja::Environment;
use minijinja::Source;
use minijinja::AutoEscape;
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

    // let source = fs::read_to_string("./content/index.neo").unwrap();
    // let payload = parse(source.as_str()).unwrap().1;
    // let output = render_template(payload, env, "main.html");
    // fs::write("./site/index.html", output).unwrap();

    dbg!("done");
}

fn create_env(path: &str) -> Environment<'static> {
    let mut env = Environment::new();
    env.set_source(Source::from_path(path));
    env.set_auto_escape_callback(|name| {
    if matches!(name.rsplit('.').next().unwrap_or(""), "html" | "jinja") {
        AutoEscape::Html
    } else {
        AutoEscape::None
    }
});
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
    let env = create_env("./templates");
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
            if p.extension().unwrap() == "neo" {
                let html_path = dest_path.with_extension("html");
                println!("PROCESSING: {}", p.as_os_str().to_str().unwrap());
                let source = fs::read_to_string(p.as_os_str().to_str().unwrap()).unwrap();
                let payload = parse(source.as_str()).unwrap().1;
                let output = render_template(payload, env.clone(), "main.jinja");
                fs::write(html_path, output).unwrap();
            }
            // NOTE: for now, just always copy the source files over to
            // easy examples. That will be put behind a config flag
            // with the default being off.
            fs::copy(p, dest_path);
        }
    }
    Ok(())
}
