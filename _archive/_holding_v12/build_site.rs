use crate::files::Files;
use crate::source_file::SourceFile;
use minijinja::context;
use minijinja::Environment;
use minijinja::Source;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn build_site() {
    println!("Making the site");

    let template_dir = PathBuf::from("templates");
    let content_dir = PathBuf::from("content");
    let site_root_dir = PathBuf::from("site");

    let mut env = Environment::new();
    env.set_source(Source::from_path(template_dir));

    let mut source_files = Files { files: vec![] };


    for entry in WalkDir::new(&content_dir).into_iter() {
        let initial_path =
            entry.unwrap().path().to_path_buf();
        if let Some(ext) = initial_path.extension() {
            if ext == "neo" {
                println!("{}", &initial_path.display());
                let sf = SourceFile {
                    source_path: initial_path
                        .clone()
                        .strip_prefix(&content_dir)
                        .unwrap()
                        .to_path_buf(),
                    source_data: fs::read_to_string(
                        initial_path,
                    )
                    .unwrap(),
                };
                source_files.files.push(sf);
            }
        }
    }

    source_files.files.iter().for_each(|source_file| {
        println!("{}", &source_file.source_path.display());
        let wrapper = env
            .get_template(&source_file.template().unwrap())
            .unwrap();
        let output = wrapper
            .render(context!(
                title => source_file.title(),
                // posts => source_files.all_posts(),
                content => source_file.content(),
            ))
            .unwrap();
        let mut file_path = site_root_dir.clone();
        file_path.push(source_file.output_path().unwrap());
        let dir_path = file_path.parent().unwrap();
        fs::create_dir_all(dir_path).unwrap();
        fs::write(file_path, output).unwrap();
    });
}
