use crate::sections::sections;
use crate::source_file::template::template;
use crate::source_file::title::title;
use crate::source_file::SourceFile;
use minijinja::context;
use minijinja::Environment;
use minijinja::Source;
use std::fs;
use std::path::PathBuf;
use std::vec;
use walkdir::WalkDir;

pub fn build_site() {
    println!("Building the site");

    let template_dir = PathBuf::from("templates");
    let content_dir = PathBuf::from("content");
    let site_root_dir = PathBuf::from("site");
    let mut env = Environment::new();
    env.set_source(Source::from_path(template_dir));
    let mut source_files: Vec<SourceFile> = vec![];

    for entry in WalkDir::new(&content_dir).into_iter() {
        let initial_path = entry.unwrap().path().to_path_buf();
        if let Some(ext) = initial_path.extension() {
            if ext == "neo" {
                println!("Loading: {}", &initial_path.display());
                let sf = SourceFile {
                    source_path: initial_path
                        .clone()
                        .strip_prefix(&content_dir)
                        .unwrap()
                        .to_path_buf(),
                    source_data: fs::read_to_string(initial_path).unwrap(),
                };
                source_files.push(sf);
            }
        }
    }

    source_files.iter().for_each(|source_file| {
        println!("Outputting: {}", &source_file.source_path.display());
        let wrapper = env.get_template(
            template(&source_file.source_data).unwrap().1.as_str(),
        );
        let the_title = title(&source_file.source_data).unwrap().1;
        let the_content = sections(&source_file.source_data).unwrap().1;

        dbg!(&the_title);
        let output = wrapper
            .unwrap()
            .render(context!(
                 title => the_title,
                content => the_content
            ))
            .unwrap();
        let mut file_path = site_root_dir.clone();
        file_path.push(&source_file.source_path);
        file_path.set_extension("html");
        let dir_path = file_path.parent().unwrap();
        fs::create_dir_all(dir_path).unwrap();
        fs::write(file_path, output).unwrap();
    });

    //
}
