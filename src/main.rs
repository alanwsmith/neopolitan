#![allow(warnings)]
use minijinja::{context, Environment};
use neopolitan::structure::structure;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut env = Environment::new();
    let base_template = fs::read_to_string("templates/post/base.html").unwrap();
    env.add_template("base", base_template.as_str());
    let post_base = env.get_template("base").unwrap();
    let source = fs::read_to_string("content/_sample_alfa.neo").unwrap();
    let structure = structure(source.as_str()).unwrap().1;
    println!(
        "{}",
        post_base.render(context!(wrapper => &structure)).unwrap()
    );
}
