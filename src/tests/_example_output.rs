// use crate::block::block::*;
// use crate::content::content::*;
use crate::parse::parse;
// use crate::section::section::*;
// // use crate::section::section_attributes::*;
use crate::wrapper::wrapper::*;
use minijinja::context;
use minijinja::Environment;
use minijinja::Source;
use std::fs;
// use serde::Serialize;

#[test]
fn example_output_1() {
    let env = create_env("./src/templates");
    let lines = vec![
        "-> title",
        "",
        "Smoke Test Alfa",
        "",
        "With a follow up paragraph",
        "with multiple source lines",
        "",
        "-> p",
        "",
        "the book cover",
        "",
        "random string",
        "with content",
        "",
        "quick <<link|example.com|brown>> fox",
        "Open the crate but don't break the glass.",
        "",
        "The ripe `taste of cheese`` improves with age.",
        "The <<strong|bark of the pine>> tree was shiny and dark.",
        "Split the log with a quick, sharp blow.",
        "He ordered peach pie with ice cream.",
        "",
        "Weave the carpet on the right hand side.",
        "The cup cracked and spilled its contents.",
        "Pluck the bright rose without leaves.",
    ]
    .join("\n");
    let source = lines.as_str();
    let payload = parse(source).unwrap().1;
    let output = render_template(payload, env, "basic_test.html");
    fs::write("./site/test_alfa.html", output).unwrap();
    // dbg!(output);
}

fn create_env(path: &str) -> Environment<'static> {
    let mut env = Environment::new();
    env.set_source(Source::from_path(path));
    env
}

fn render_template(payload: Wrapper, env: Environment, template: &str) -> String {
    let tmpl = env.get_template(template).unwrap();
    tmpl.render(context!(payload => &payload))
        .unwrap()
        .to_string()
}
