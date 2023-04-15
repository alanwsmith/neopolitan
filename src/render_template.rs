use crate::wrapper::wrapper::Wrapper;
use minijinja::context;
use minijinja::Environment;

pub fn render_template(payload: Wrapper, env: Environment, template: &str) -> String {
    let tmpl = env.get_template(template).unwrap();
    tmpl.render(context!(payload => &payload))
        .unwrap()
        .to_string()
}
