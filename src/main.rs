use core::fmt::Error;
use miette::Result;
use std::time::Duration;
use neopolitan::universe::create_env::create_env;
use neopolitan::universe::universe::Universe;
use std::path::PathBuf;

use watchexec::action::Action;
use watchexec::action::Outcome;
use watchexec::config::InitConfig;
use watchexec::config::RuntimeConfig;
use watchexec::Watchexec;
use watchexec_signals::Signal;

fn main() {
    println!("Starting process");
    watch_files().unwrap();
}


#[tokio::main]
pub async fn watch_files() -> Result<()> {
    println!("Starting watcher");

    // // PROD alanwsmith.com
    // let templates_dir = "/Users/alan/workshop/alanwsmith.com/templates";
    // let output_root = PathBuf::from("/Users/alan/workshop/alanwsmith.com/site/posts");
    // let content_dir = PathBuf::from("/Users/alan/workshop/grimoire_org_to_neo_files/content");

    // PROD neopolitan.alanwsmith.com 
    let templates_dir = "./templates";
    let content_dir = PathBuf::from("./content");
    let output_root = PathBuf::from("./site");

    // // DEV
    // let templates_dir = "./site_dev/templates";
    // let content_dir = PathBuf::from("./site_dev/content");
    // let output_root = PathBuf::from("./site_dev/build");

    let mut u = Universe::new();

    u.content_dir = Some(content_dir.canonicalize().unwrap());
    u.output_root = Some(output_root.canonicalize().unwrap());
    u.env = Some(create_env(templates_dir));
    u.load_raw_data().unwrap();
    u.output_files();
    // u.output_index_file();
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
