// use neopolitan::page::Page;
use minijinja::{context, Environment};
use neopolitan::get_structure::get_structure;
use std::fs;
use std::path::PathBuf;

fn main() {
    // hard coded input_path to be run from
    // the Cargo.toml directory
    let mut input_path = PathBuf::new();
    input_path.push("src");
    input_path.push("_small_sample");
    input_path.set_extension("neo");
    let mut output_path = PathBuf::new();
    output_path.push("/");
    output_path.push("Users");
    output_path.push("alan");
    output_path.push("Desktop");
    output_path.push("neopolitan_test.html");

    let template_path = PathBuf::from("src/_sample_template.html");

    let mut env = Environment::new();
    let file_string = fs::read_to_string(template_path).unwrap();
    env.add_template("hello", file_string.as_str()).unwrap();
    // let tmpl = env.get_template("hello").unwrap();

    let source = fs::read_to_string(input_path).expect("Could not open file");
    let structure = get_structure(source.as_str());

    // println!(
    //     "{}",
    //     tmpl.render(context!(page => &structure.unwrap().1))
    //         .unwrap()
    // );

    dbg!(&structure.unwrap().1);
}
