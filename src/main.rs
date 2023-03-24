#![allow(warnings)]
use minijinja::{context, Environment};
use neopolitan::structure::structure;
use std::fs;

fn main() {
    let templates = vec![
        "CodeSection",
        "H1",
        "InlineCode",
        "Link",
        "ListItem",
        "ListSection",
        "P",
        "ParagraphSection",
        "Post",
        "Strong",
        "Text",
        "TitleSection",
    ];

    let mut env = Environment::new();

    let mut string_vec_hack: Vec<(String, String)> = vec![];
    templates.iter().for_each(|t| {
        string_vec_hack.push((
            t.to_string(),
            fs::read_to_string(format!("src/_templates/post/{}.html", t)).unwrap(),
        ));
    });

    string_vec_hack.iter().for_each(|s| {
        env.add_template(&s.0, &s.1);
    });

    let source = fs::read_to_string("src/_content/_sample.neo").unwrap();
    let structure = structure(source.as_str()).unwrap().1;
    dbg!(&structure);
    let post_base = &env.get_template("Post").unwrap();

    fs::write(
        "site/sample.html",
        post_base.render(context!(wrapper => &structure)).unwrap(),
    )
    .unwrap();
}
