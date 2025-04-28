// #![allow(warnings)]
// // use minijinja::AutoEscape;
// use minijinja::context;
// use minijinja::Environment;
// use minijinja::Source;
// use neopolitan::source_file::source_file::SourceFile;
// use std::fs;
// use walkdir::Error;
// use walkdir::WalkDir;

use neopolitan::build_site::build_site;

fn main() {
    println!("Starting process");
    // watch_files().unwrap();
    build_site();
}

// #[derive(Debug)]
// struct Files {
//     pub content_dir: Option<PathBuf>,
//     pub files: Vec<SourceFile>,
// }

// impl Files {
//     pub fn posts(&self) -> Vec<(PathBuf, String)> {
//         let mut post_data = vec![];
//         self.files.iter().for_each(|file| {
//             let mut url_path = PathBuf::from("/");
//             url_path.push(file.raw_path.clone().unwrap());
//             url_path.set_extension("html");
//             post_data.push((url_path, String::from("Title Goes Here")));
//             ()
//         });
//         post_data
//     }
// }

// impl Files {
//     pub fn load_files(&mut self) -> Result<(), Error> {
//         for entry in WalkDir::new(&self.content_dir.as_ref().unwrap()).into_iter() {
//             let p = entry?.path().to_path_buf();
//             if let Some(ext) = p.extension() {
//                 if ext == "neo" {
//                     let mut sf = SourceFile::new();
//                     sf.raw = Some(fs::read_to_string(&p.to_str().unwrap()).unwrap());
//                     sf.raw_path = Some(
//                         p.strip_prefix(&self.content_dir.as_ref().unwrap())
//                             .unwrap()
//                             .to_path_buf(),
//                     );
//                     let _ = &self.files.push(sf);
//                 }
//             }
//         }
//         Ok(())
//     }
// }

// use core::fmt::Error;
// use miette::Result;
// use neopolitan::universe::create_env::create_env;
// use neopolitan::universe::universe::Universe;
// use std::time::Duration;

// use watchexec::action::Action;
// use watchexec::action::Outcome;
// use watchexec::config::InitConfig;
// use watchexec::config::RuntimeConfig;
// use watchexec::Watchexec;
// use watchexec_signals::Signal;

// // TODO: Make site directories
// fn make_site() {
//     println!("Making the site");
//     let mut content = Files {
//         files: vec![],
//         content_dir: None,
//     };
//     let mut env = Environment::new();
//     env.set_source(Source::from_path(PathBuf::from("./templates")));
//     let wrapper = env.get_template("home_page.j2").unwrap();
//     content.content_dir = Some(PathBuf::from("./content"));
//     let _ = content.load_files();
//     content.files.iter().for_each(|file| {
//         let mut output_path = PathBuf::from("./site");
//         output_path.push(file.raw_path.as_ref().unwrap());
//         output_path.set_extension("html");
//         let output = wrapper
//             .render(context!(
//                 content => "ALFA BRAVO CHARLIE",
//                 posts => content.posts(),
//             ))
//             .unwrap()
//             .to_string();
//         fs::write(output_path, output).unwrap();
//         ()
//     });
// }

// #[tokio::main]
// pub async fn watch_files() -> Result<()> {
//     println!("Starting watcher");
//     // // PROD alanwsmith.com
//     // let templates_dir = "/Users/alan/workshop/alanwsmith.com/templates";
//     // let output_root = PathBuf::from("/Users/alan/workshop/alanwsmith.com/site/posts");
//     // let content_dir = PathBuf::from("/Users/alan/workshop/grimoire_org_to_neo_files/content");
//     // PROD neopolitan.alanwsmith.com
//     let templates_dir = "./templates";
//     let content_dir = PathBuf::from("./content");
//     let output_root = PathBuf::from("./site");
//     // // DEV
//     // let templates_dir = "./site_dev/templates";
//     // let content_dir = PathBuf::from("./site_dev/content");
//     // let output_root = PathBuf::from("./site_dev/build");
//     let mut u = Universe::new();
//     u.content_dir = Some(content_dir.canonicalize().unwrap());
//     u.output_root = Some(output_root.canonicalize().unwrap());
//     u.env = Some(create_env(templates_dir));
//     u.load_raw_data().unwrap();
//     u.output_files();
//     // u.output_index_file();
//     let init = InitConfig::default();
//     let mut runtime = RuntimeConfig::default();
//     runtime.pathset(["./content"]);
//     runtime.action_throttle(Duration::new(0, 100000));
//     runtime.on_action(move |action: Action| {
//         let mut stop: bool = false;
//         let mut paths: Vec<PathBuf> = vec![];
//         action.events.iter().for_each(|event| {
//             event.signals().for_each(|sig| match sig {
//                 Signal::Interrupt => {
//                     println!("Caught Interrupt: Stopping");
//                     stop = true;
//                 }
//                 _ => {}
//             });
//             event
//                 .paths()
//                 .for_each(|path| paths.push(path.0.to_path_buf()));
//         });
//         if stop {
//             action.outcome(Outcome::Exit);
//         }
//         paths.dedup();
//         paths.iter().for_each(|path| {
//             println!("File change: {}", path.display());
//             u.output_file(path.clone().to_path_buf());
//         });
//         async move { Ok::<(), Error>(()) }
//     });
//     let we = Watchexec::new(init, runtime)?;
//     we.main().await.unwrap().unwrap();
//     Ok(())
// }
