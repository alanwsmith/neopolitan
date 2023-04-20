use neopolitan::universe::create_env::create_env;
use neopolitan::universe::universe::Universe;
use std::path::PathBuf;

fn main() {
    println!("Starting build");
    let mut u = Universe::new();
    u.env = Some(create_env("./site/templates"));
    u.source_dir = Some(PathBuf::from("./site/content"));
    u.dest_dir = Some(PathBuf::from("./site/html"));
    u.load_files().unwrap();
    u.output_files();
}
