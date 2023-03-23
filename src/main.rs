#![allow(warnings)]
use minijinja::{context, Environment};
use neopolitan::structure::structure;
use std::fs;

fn main() {
    let mut env = Environment::new();

    let post_base_template = fs::read_to_string("src/_templates/post/base.html").unwrap();
    &env.add_template("Post", post_base_template.as_str());

    let post_title_template = fs::read_to_string("src/_templates/post/title.html").unwrap();
    &env.add_template("Title", post_title_template.as_str());

    let post_h1_template = fs::read_to_string("src/_templates/post/H1.html").unwrap();
    &env.add_template("H1", post_h1_template.as_str());

    let post_text_template = fs::read_to_string("src/_templates/post/Text.html").unwrap();
    &env.add_template("Text", post_text_template.as_str());

    let source = fs::read_to_string("src/_content/_sample_alfa.neo").unwrap();
    let structure = structure(source.as_str()).unwrap().1;
    let post_base = &env.get_template("Post").unwrap();
    println!(
        "{}",
        post_base.render(context!(wrapper => &structure)).unwrap()
    );
}
