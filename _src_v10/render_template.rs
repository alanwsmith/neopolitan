// use crate::wrapper::wrapper::Wrapper;
use crate::universe::Universe;
use minijinja::context;
use minijinja::Environment;

pub fn render_template(
    universe: &Universe,
    current_index: u32,
    env: Environment,
    template: &str,
) -> String {
    let tmpl = env.get_template(template).unwrap();
    tmpl.render(context!(universe => universe, current_index => current_index))
        .unwrap()
        .to_string()
}

// pub fn render_template(payload: Wrapper, env: Environment, template: &str) -> String {
//     let tmpl = env.get_template(template).unwrap();
//     tmpl.render(context!(payload => &payload))
//         .unwrap()
//         .to_string()
// }
