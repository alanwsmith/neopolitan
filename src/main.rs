use neopolitan::universe::create_env::create_env;
use neopolitan::universe::universe::Universe;
use std::path::PathBuf;

fn main() {
    println!("Starting build");
    let mut u = Universe::new();
    u.env = Some(create_env("./src/tests/templates"));
    u.env
        .unwrap()
        .add_template("wrapper.j2", "./templates/wrapper.j2");
    u.source_dir = Some(PathBuf::from("./content"));
    u.dest_dir = Some(PathBuf::from("./sites/default"));
    u.load_files().unwrap();
    u.output_files();
}
