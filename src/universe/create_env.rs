use minijinja::AutoEscape;
use minijinja::Environment;
use minijinja::Source;

pub fn create_env(path: &str) -> Environment<'static> {
    let mut env = Environment::new();
    // dbg!(&path);
    env.set_source(Source::from_path(path));
    // dbg!(&env);
    env.set_auto_escape_callback(|name| {
        if matches!(
            name.rsplit('.').next().unwrap_or(""),
            "html" | "jinja" | "j2"
        ) {
            AutoEscape::Html
        } else {
            AutoEscape::None
        }
    });
    env
}
