use minijinja::context;
use minijinja::Environment;
use minijinja::Source;
use neopolitan::parse::parse;
use neopolitan::wrapper::wrapper::*;
use std::fs;

fn main() {
    let env = create_env("./templates");
    let source = fs::read_to_string("./content/index.neo").unwrap();
    let payload = parse(source.as_str()).unwrap().1;
    let output = render_template(payload, env, "main.html");
    fs::write("./site/index.html", output).unwrap();
    dbg!("done");
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
