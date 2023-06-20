// #![allow(warnings)]
// use minijinja::context;
// use minijinja::Environment;
// use minijinja::Source;
// use crate::source_file::source_file::SourceFile;
// use crate::files::files::Files;
// use std::fs;
// use std::path::PathBuf;
//use walkdir::Error;
// use walkdir::WalkDir;

use std::fs;

pub fn build_site() {
    println!("Making the site");
    fs::create_dir("site").unwrap();

    let source = r#"
        <!DOCTYPE html>
        <html>
            <head><title>Alfa Bravo</title></head>
        </html>"#;

    fs::write("site/index.html", source).unwrap();

    // let mut content = Files {
    //     files: vec![],
    //     content_dir: None,
    // };
    // let mut env = Environment::new();
    // env.set_source(Source::from_path(PathBuf::from("./templates")));
    // let wrapper = env.get_template("home_page.j2").unwrap();
    // content.content_dir = Some(PathBuf::from("./content"));
    // let _ = content.load_files();
    // content.files.iter().for_each(|file| {
    //     let mut output_path = PathBuf::from("./site");
    //     output_path.push(file.raw_path.as_ref().unwrap());
    //     output_path.set_extension("html");
    //     let output = wrapper
    //         .render(context!(
    //             content => "ALFA BRAVO CHARLIE",
    //             posts => content.posts(),
    //         ))
    //         .unwrap()
    //         .to_string();
    //     fs::write(output_path, output).unwrap();
    //     ()
    // });
}
