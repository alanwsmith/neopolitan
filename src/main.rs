use neopolitan::helpers::load_assets::load_assets;
use neopolitan::universe::create_env::create_env;
use neopolitan::universe::universe::Universe;
// use std::path::Path;
use std::path::PathBuf;

fn main() {
    println!("Starting build");
    let templates_dir = "./site/templates";
    let assets_dir = "./site/assets/";
    let content_dir = "./site/content";
    let build_dir = "./site/build";
    load_assets(assets_dir, build_dir).unwrap();

    let mut u = Universe::new();
    u.env = Some(create_env(templates_dir));
    u.assets_dir = Some(PathBuf::from(assets_dir));
    u.source_dir = Some(PathBuf::from(content_dir));
    u.dest_dir = Some(PathBuf::from(build_dir));
    u.load_files().unwrap();
    u.output_files();
}
