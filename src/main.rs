#![allow(warnings)]
use neopolitan::page_builder::PageBuilder;
use std::fs;

fn main() {
    let mut pb = PageBuilder::new();

    pb.input = Some(
        fs::read_to_string("samples/alfa.neo")
            .unwrap(),
    );

    dbg!(pb.output());

    // for line in pb.content(&input_alfa) {
    //     println!("{}", line);
    // }
}
