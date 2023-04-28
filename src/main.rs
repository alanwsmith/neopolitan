use neopolitan::helpers::load_assets::load_assets;
// use neopolitan::helpers::run_preflight::run_preflight;
use neopolitan::universe::create_env::create_env;
use neopolitan::universe::universe::Universe;
// use std::path::Path;
use std::path::PathBuf;

use core::fmt::Error;
use core::time::Duration;
// use miette::Result;
use watchexec::{
    action::Action,
    action::Outcome,
    config::{InitConfig, RuntimeConfig},
    Watchexec,
};
use watchexec_events::filekind::FileEventKind;
use watchexec_events::filekind::ModifyKind;
use watchexec_events::Tag;

// NOTE: This panics on errors for now.
// TBD on when/how to address that

#[tokio::main]
async fn main() -> Result<(), Error> {
    generate_site();
    println!("Starting process");
    let content_dir = "/Users/alan/workshop/grimoire_org_to_neo_files/step-01";
    let init = InitConfig::default();
    let mut runtime = RuntimeConfig::default();
    runtime.pathset([content_dir]);
    runtime.action_throttle(Duration::new(0, 100000));
    let we = Watchexec::new(init, runtime.clone()).unwrap();
    runtime.on_action(move |action: Action| {
        async move {
            let mut events: Vec<PathBuf> = vec![];
            for event in action.events.iter() {
                let mut trigger: bool = false;
                let mut file_path: Option<PathBuf> = None;
                event.tags.iter().for_each(|tag| match tag {
                    Tag::Path { path, .. } => {
                        file_path = Some(path.to_path_buf());
                        events.push(file_path.clone().unwrap());
                    }
                    Tag::FileEventKind(event_kind) => match event_kind {
                        FileEventKind::Create(_) => {
                            trigger = true;
                        }
                        FileEventKind::Modify(modify_kind) => match modify_kind {
                            ModifyKind::Data(_) => {
                                trigger = true;
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                });
            }
            // events.dedup();
            generate_site();
            // events.iter().for_each(|p| do_something(p.to_path_buf()));
            action.outcome(Outcome::DoNothing);
            Ok::<(), Error>(())
        }
    });
    we.reconfigure(runtime).unwrap();
    we.main().await.unwrap().unwrap();
    Ok(())
}

fn generate_site() {
    println!("Starting build");
    let templates_dir = "./site/templates";
    let assets_dir = "./site/assets/";

    // let content_dir = "./site/content";
    let content_dir = "/Users/alan/workshop/grimoire_org_to_neo_files/step-01";

    // let build_dir = "./site/build";
    let build_dir = "/Users/alan/workshop/grimoire_org_to_neo_files/test_build";

    // Be careful with the preflight right now
    // it updates the contents directory which
    // triggers cargo watch if it's pointed
    // at it which turns into a loop. TODO:
    // - ignore the directory that gets updated
    // run_preflight().unwrap();
    load_assets(assets_dir, build_dir).unwrap();

    let mut u = Universe::new();
    u.env = Some(create_env(templates_dir));
    u.assets_dir = Some(PathBuf::from(assets_dir));
    u.source_dir = Some(PathBuf::from(content_dir));
    u.dest_dir = Some(PathBuf::from(build_dir));
    u.load_files().unwrap();
    u.output_files();
}
