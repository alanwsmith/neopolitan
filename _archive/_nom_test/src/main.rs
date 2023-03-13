#![allow(warnings)]
use dirs;
use neopolitan::page_builder::PageBuilder;
use std::fs;

fn main() {
    let mut pb = PageBuilder::new();

    let name = "charlie";

    let mut input_path = dirs::home_dir().unwrap();
    input_path.push("workshop");
    input_path.push("neopolitan");
    input_path.push("samples");
    input_path.push(format!("{}.neo", name).as_str());

    let mut output_path = dirs::home_dir().unwrap();
    output_path.push("workshop");
    output_path.push("neopolitan-tests");
    output_path.push("site");
    output_path.push(format!("{}.html", name).as_str());

    pb.input =
        Some(fs::read_to_string(input_path).unwrap());

    fs::write(output_path, &pb.output());

    // println!("{}", (&pb.output()));
}
