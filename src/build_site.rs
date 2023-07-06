// use crate::files::Files;
use crate::source_file::SourceFile;
use crate::template::template;
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
        let output = wrapper
            .unwrap()
            .render(context!(
            //             title => source_file.title(),
            //             // posts => source_files.all_posts(),
            //             content => source_file.content(),
                     ))
            .unwrap();
        dbg!(output);
        //     let mut file_path = site_root_dir.clone();
        //     file_path.push(source_file.output_path().unwrap());
        //     let dir_path = file_path.parent().unwrap();
        //     fs::create_dir_all(dir_path).unwrap();
        //     fs::write(file_path, output).unwrap();
    });

    //
}
