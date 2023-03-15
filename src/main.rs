use neopolitan::prep::prep;
use std::fs;

fn main() {
    let source = fs::read_to_string("test_samples/input/0001.neo").unwrap();
    let blocks = prep(source.as_str()).unwrap();
    dbg!(blocks);
    println!("under construction");
}
