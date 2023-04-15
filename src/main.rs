#![allow(warnings)]
use minijinja::context;
use minijinja::AutoEscape;
use minijinja::Environment;
use minijinja::Source;
use neopolitan::block::block::Block::RawContent;
use neopolitan::create_env::create_env;
use neopolitan::parse::parse;
use neopolitan::render_template::render_template;
use neopolitan::section::section::*;
use neopolitan::wrapper::wrapper::*;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;
use walkdir::{DirEntry, Error, WalkDir};

fn main() {
    do_copy("./content", "./site").unwrap();
    println!("PROCESS COMPLETE");
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn do_copy(source_dir: &str, dest_dir: &str) -> Result<(), Error> {
    let env = create_env("./templates");
    let walker = WalkDir::new(source_dir).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let p = entry?.path().to_path_buf();
        let sub_path = &p.strip_prefix(source_dir);
        let mut dest_path = PathBuf::from(dest_dir);
        dest_path.push(sub_path.as_ref().unwrap());
        if p.is_dir() {
            if !dest_path.exists() {
                create_dir_all(dest_path).unwrap();
            }
        } else {
            if p.extension().unwrap() == "neo" {
                let html_path = dest_path.with_extension("html");
                println!("PROCESSING: {}", p.as_os_str().to_str().unwrap());
                let source = fs::read_to_string(p.as_os_str().to_str().unwrap()).unwrap();
                let mut payload = parse(source.as_str()).unwrap().1;

                // NOTE: This has been problematic to implement. trying
                // another way.
                // // NOTE: This is a hack for the neoexample sections
                // // to add parsed content for the documentation.
                // // Will examine to see if it cause performance issues.
                // // Also, I'm not sure this is an idiomati way to
                // // add data to an enum.
                // payload = match payload {
                //     Wrapper::Page { children } => {
                //         let mut new_children: Vec<Section> = vec![];
                //         for sec in children.unwrap() {
                //             match sec {
                //                 Section::NeoExampleStartEndSection {
                //                     children,
                //                     raw,
                //                     attributes,
                //                 } => {
                //                     // let mut new_raw: Option<Block>;
                //                     let raw_content = raw.as_ref().unwrap();
                //                     match raw_content {
                //                         RawContent { text } => {
                //                             let sub_page = parse((text.as_ref().unwrap()));
                //                             // dbg!(sub_page);
                //                             match sub_page.unwrap().1 {
                //                                 Wrapper::Page { children } => {
                //                                     for tmp_sec in children.as_ref().unwrap() {
                //                                         match tmp_sec {
                //                                             Section::AsideSection {
                //                                                 children,
                //                                                 ..
                //                                             } => {
                //                                                 new_children.push(
                //                                                               Section::NeoExampleStartEndSection {
                //                                                                   attributes,
                //                                                                   children: None,
                //                                                                   raw,
                //                                                               },
                //                                                     );
                //                                                 // dbg! {children};
                //                                                 ()
                //                                             }
                //                                             _ => (), //         children,
                //                                                      //         raw,
                //                                                      //         attributes,
                //                                                      //     } => {
                //                                                      //         new_children.push(
                //                                                      //         Section::NeoExampleStartEndSection {
                //                                                      //             attributes,
                //                                                      //             children: None,
                //                                                      //             raw,
                //                                                      //         },
                //                                                      //     );
                //                                                      //         ()
                //                                                      //     }
                //                                         }
                //                                     }
                //                                 }
                //                             }
                //                             ()
                //                         }
                //                         _ => (),
                //                     }
                //                 }
                //                 _ => new_children.push(sec),
                //             }
                //         }
                //         Wrapper::Page {
                //             children: Some(new_children),
                //         }
                //     }
                // };

                let output = render_template(payload, env.clone(), "main.jinja");
                fs::write(html_path, output).unwrap();
            }
            // NOTE: for now, just always copy the source files over to
            // easy examples. That will be put behind a config flag
            // with the default being off.
            fs::copy(p, dest_path);
        }
    }
    Ok(())
}
