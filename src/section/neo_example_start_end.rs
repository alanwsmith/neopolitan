use crate::block::block::*;
use crate::create_env::create_env;
use crate::parse::parse;
use crate::render_template::render_template;
use crate::section::attributes_for_section::*;
use crate::section::section::*;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

// This one is a little different. It does its
// own render of the data so it can output
// it as an example

pub fn neo_example_start_end(source: &str) -> IResult<&str, Section> {
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };

    let env = create_env("./templates");
    let payload = parse(source.trim()).unwrap().1;
    let html = render_template(payload, env.clone(), "neoexample_extract.jinja");
    let raw = if source.trim().is_empty() {
        None
    } else {
        Some(Block::RawContent {
            text: Some(source.trim().to_string()),
        })
    };

    Ok((
        source,
        Section::NeoExampleStartEndSection {
            attributes,
            html: Some(html),
            raw,
        },
    ))
}

#[cfg(test)]

mod test {
    use crate::block::block::*;
    //use crate::content::content::*;
    use crate::parse::parse;
    // use crate::section::attributes_for_section::*;
    use crate::section::section::*;
    use crate::wrapper::wrapper::*;

    #[test]
    fn basic_code_start_end() {
        let lines = vec![
            "-> startneoexample",
            "",
            "-> aside",
            "",
            "the text",
            "",
            "-> endneoexample",
        ]
        .join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::NeoExampleStartEndSection {
                attributes: None,
                html: Some("<aside><p>the text</p></aside>".to_string()),
                raw: Some(Block::RawContent {
                    text: Some("-> aside\n\nthe text".to_string()),
                }),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}
