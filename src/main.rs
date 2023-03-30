#![allow(warnings)]
use minijinja::{context, Environment};
use neopolitan::structure::structure;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
struct Attributes {
    date: Option<String>,
    id: Option<String>,
    // underscore to avoid name collision
    site: Option<String>,
    type_: Option<String>,
    status: Option<String>,
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes {
            date: None,
            id: None,
            site: None,
            type_: None,
            status: None,
        }
    }
}

#[test]
fn test_get_attributes() {
    let lines = [
        "",
        "-> attributes",
        ">> date: 2023-01-02 14:15:16",
        ">> id: 1234asdf",
        ">> site: aws",
        ">> type: post",
        ">> status: published",
        "",
    ];
    let source = lines.join("\n");
    let attributes = get_attributes(source.as_str()).unwrap().1;
    let expected = Attributes {
        date: Some("2023-01-02 14:15:16".to_string()),
        id: Some("1234asdf".to_string()),
        site: Some("aws".to_string()),
        type_: Some("post".to_string()),
        status: Some("published".to_string()),
    };

    assert_eq!(expected, attributes);
}
fn get_attributes(source: &str) -> IResult<&str, Attributes> {
    let mut attributes = Attributes::new();

    // Find the attributes section
    let (r, c) = alt((take_until("\n-> attributes\n"), rest))(source)?;
    let (r, c) = tag("\n-> attributes\n")(r)?;

    let (a, b) = alt((
        tuple((take_until(">> date: "), tag(">> date: "), not_line_ending)).map(|x| x.2),
        rest,
    ))(r)?;
    if !a.is_empty() {
        attributes.date = Some(b.to_string());
    }

    let (a, b) = alt((
        tuple((take_until(">> id: "), tag(">> id: "), not_line_ending)).map(|x| x.2),
        rest,
    ))(r)?;
    if !a.is_empty() {
        attributes.id = Some(b.to_string());
    }

    let (a, b) = alt((
        tuple((take_until(">> site: "), tag(">> site: "), not_line_ending)).map(|x| x.2),
        rest,
    ))(r)?;
    if !a.is_empty() {
        attributes.site = Some(b.to_string());
    }

    let (a, b) = alt((
        tuple((take_until(">> type: "), tag(">> type: "), not_line_ending)).map(|x| x.2),
        rest,
    ))(r)?;
    if !a.is_empty() {
        attributes.type_ = Some(b.to_string());
    }

    let (a, b) = alt((
        tuple((
            take_until(">> status: "),
            tag(">> status: "),
            not_line_ending,
        ))
        .map(|x| x.2),
        rest,
    ))(r)?;
    if !a.is_empty() {
        attributes.status = Some(b.to_string());
    }

    Ok(("", attributes))
}

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

    let paths = fs::read_dir("/Users/alan/workshop/grimoire_org_to_neo_files/step-01").unwrap();
    for path in paths {
        let input_path = path.unwrap().path().clone();
        match input_path.extension() {
            Some(ext) => {
                if ext == "neo" {
                    let mut output_path =
                        PathBuf::from("/Users/alan/workshop/alanwsmith.com/site/posts");
                    output_path.push(input_path.file_name().unwrap());
                    let source = fs::read_to_string(input_path.clone()).unwrap();
                    // let source = make_contents(input.as_str());
                    let attributes = get_attributes(source.as_str());
                    match attributes {
                        Ok(ref a) => {
                            match &a.1.status {
                                None => (),
                                Some(s) => {
                                    if s == "published" {
                                        dbg!(output_path);
                                        let structure = structure(source.as_str());
                                        dbg!(s);
                                    }
                                    ()
                                }
                            }
                            ()
                        }
                        Err(_) => (),
                    }
                    // dbg!(attributes);

                    // fs::write(output_path, &contents);
                }
            }
            None => {}
        }
    }

    // let file_paths = file_paths();
    // for file_path in file_paths.iter() {
    //     dbg!(file_path);
    // }

    // let source = fs::read_to_string("src/_content/_sample.neo").unwrap();
    // let structure = structure(source.as_str()).unwrap().1;
    // // dbg!(&structure);
    // let post_base = &env.get_template("Post").unwrap();

    // fs::write(
    //     "site/sample.html",
    //     post_base.render(context!(wrapper => &structure)).unwrap(),
    // )
    // .unwrap();
}

// fn file_paths() -> Vec<PathBuf> {
//     let paths: Vec<PathBuf> =
//         fs::read_dir("/Users/alan/workshop/grimoire_org_to_neo_files/step-01")
//             .unwrap()
//             .into_iter()
//             .map(|p| p.unwrap())
//             .filter(|p| {
//                 p.file_name()
//                     .to_str()
//                     .map(|fname| !fname.starts_with("."))
//                     .unwrap()
//             })
//             .map(|p| p.path())
//             .collect();
//     paths
// }
