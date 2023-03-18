// use neopolitan::page::Page;
use neopolitan::get_structure::get_structure;
use std::fs;
use std::path::PathBuf;

fn main() {
    // hard coded path to be run from
    // the Cargo.toml directory
    let mut path = PathBuf::new();
    path.push("src");
    path.push("_sample");
    path.set_extension("neo");
    let source = fs::read_to_string(path).expect("Could not open file");
    let structure = get_structure(source.as_str());
    dbg!(&structure);
}
