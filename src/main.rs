use core::fmt::Error;
use miette::Result;
use std::time::Duration;
// use neopolitan::helpers::load_assets::load_assets;
use neopolitan::universe::create_env::create_env;
use neopolitan::universe::universe::Universe;
use std::path::PathBuf;

use watchexec::action::Action;
use watchexec::action::Outcome;
use watchexec::config::InitConfig;
use watchexec::config::RuntimeConfig;
use watchexec::Watchexec;
use watchexec_signals::Signal;

// #[tokio::main]
// async fn main() -> Result<(), Error> {
fn main() {
    println!("Starting process");

    println!("Starting build");
    watch_files().unwrap();

    // u.assets_dir = Some(PathBuf::from(assets_dir));
    // u.source_dir = Some(PathBuf::from(content_dir));
    // u.dest_dir = Some(PathBuf::from(build_dir));
    // u.load_files().unwrap();
    // u.output_files();

    // u.watch_files();

    // let init = InitConfig::default();
    // let mut runtime = RuntimeConfig::default();
    // runtime.pathset([content_dir]);
    // let u2 = u.clone();
    // runtime.on_action(move |action: Action| async move {
    //     let mut stop: bool = false;
    //     let mut paths: Vec<PathBuf> = vec![];
    //     action.events.iter().for_each(|event| {
    //         event.signals().for_each(|sig| match sig {
    //             Signal::Interrupt => {
    //                 println!("Caught Interrupt: Stopping");
    //                 stop = true;
    //             }
    //             _ => {}
    //         });
    //         event
    //             .paths()
    //             .for_each(|path| paths.push(path.0.to_path_buf()));
    //     });
    //     if stop {
    //         action.outcome(Outcome::Exit);
    //     }
    //     paths.dedup();
    //     for path in paths.iter() {
    //         dbg!(path);
    //         // u.update_file(path);
    //         do_update(path);
    //     }
    //     // paths.iter().for_each(|path| do_update(&u));
    //     // .for_each(|path| println!("Change at {:?}", path));
    //     Ok::<(), Error>(())
    // });
    // let we = Watchexec::new(init, runtime).unwrap();
    // we.main().await.unwrap().unwrap();
    // Ok(())
}

// fn do_update(u: &PathBuf) {}

// fn generate_site(content_dir: &str) {
//     println!("Starting build");
//     let templates_dir = "./site/templates";
//     let assets_dir = "./site/assets/";
//     // let build_dir = "./site/build";
//     let build_dir = "/Users/alan/workshop/grimoire_org_to_neo_files/test_build";
//     // Be careful with the preflight right now
//     // it updates the contents directory which
//     // triggers cargo watch if it's pointed
//     // at it which turns into a loop. TODO:
//     // - ignore the directory that gets updated
//     // run_preflight().unwrap();
//     load_assets(assets_dir, build_dir).unwrap();
//     let mut u = Universe::new();
//     u.env = Some(create_env(templates_dir));
//     u.assets_dir = Some(PathBuf::from(assets_dir));
//     u.source_dir = Some(PathBuf::from(content_dir));
//     u.dest_dir = Some(PathBuf::from(build_dir));
//     u.load_files().unwrap();
//     u.output_files();
// }

#[tokio::main]
pub async fn watch_files() -> Result<()> {
    println!("Starting watcher");
    let mut u = Universe::new();
    let templates_dir = "./templates";
    //let content_dir = PathBuf::from("/Users/alan/workshop/grimoire_org_to_neo_files/step-01");
    // let output_root = PathBuf::from("/Users/alan/workshop/grimoire_org_to_neo_files/test_build");
    let content_dir = PathBuf::from("./content");
    let output_root = PathBuf::from("./site");
    u.content_dir = Some(content_dir.canonicalize().unwrap());
    u.output_root = Some(output_root.canonicalize().unwrap());
    u.env = Some(create_env(templates_dir));
    // u.find_files().unwrap();
    u.load_raw_data().unwrap();
    u.output_files();
    let init = InitConfig::default();
    let mut runtime = RuntimeConfig::default();
    runtime.pathset(["./site/content"]);
    runtime.action_throttle(Duration::new(0, 100000));
    runtime.on_action(move |action: Action| {
        let mut stop: bool = false;
        let mut paths: Vec<PathBuf> = vec![];
        action.events.iter().for_each(|event| {
            event.signals().for_each(|sig| match sig {
                Signal::Interrupt => {
                    println!("Caught Interrupt: Stopping");
                    stop = true;
                }
                _ => {}
            });
            event
                .paths()
                .for_each(|path| paths.push(path.0.to_path_buf()));
        });
        if stop {
            action.outcome(Outcome::Exit);
        }
        paths.dedup();
        paths.iter().for_each(|path| {
            println!("File change: {}", path.display());
            u.output_file(path.clone().to_path_buf());
        });
        async move { Ok::<(), Error>(()) }
    });
    let we = Watchexec::new(init, runtime)?;
    we.main().await.unwrap().unwrap();
    Ok(())
}
