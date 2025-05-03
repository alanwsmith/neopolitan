#!/usr/bin/env cargo -Zscript

---
[dependencies]
anyhow = "1.0.98"
minijinja = { version = "2.9.0", features = ["custom_syntax"] }
serde = { version = "1.0.219", features = ["derive"] }
---

use anyhow::Result;
use minijinja::{Environment, Value, context};
use minijinja::syntax::SyntaxConfig;
use serde::Serialize;
use std::path::PathBuf;

const TEMPLATE: &str = r#"
[! for item in payload.items !]
Item: [@ item @]
[! endfor !]
"#;

#[derive(Serialize)]
struct Payload {
    items: Vec<String>
}

impl Payload {
    pub fn new() -> Payload{
        let items = vec!["a", "b", "c"]
            .iter()
            .map(|i| i.to_string())
            .collect();
        Payload{
            items 
        }
    }
}

fn main() -> Result<()> {
    let output_path = PathBuf::from(
        "../src/span_attr/span_add_key_token_tests.rs"
    );
    let payload = Payload::new();
    let content = generate_output(payload)?;
    println!("{}", &content);
    std::fs::write(output_path, content)?;
    Ok(())
}

fn generate_output(payload: Payload) -> Result<String> {
    let mut env = Environment::new();
    env.set_syntax(
        SyntaxConfig::builder()
            .block_delimiters("[!", "!]")
            .variable_delimiters("[@", "@]")
            .comment_delimiters("[#", "#]")
            .build()
            .unwrap(),
    );
    env.add_template(
        "skeleton", TEMPLATE
    )?;
    let template = env.get_template("skeleton")?;
    let output = template.render(context!(
        payload => Value::from_serialize(payload)
    ))?;
    Ok(output)
}

fn write_with_mkdir(path: &PathBuf, content: &str) -> Result<()> {
  let parent_dir = path.parent().ok_or(
    std::io::Error::other("Could not get parent path")
  )?;
  std::fs::create_dir_all(parent_dir)?;
  std::fs::write(path, content)?;
  Ok(())
}
