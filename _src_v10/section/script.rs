use crate::block::block::*;
use crate::section::attributes_for_section::*;
use crate::section::section::*;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

pub fn script(source: &str) -> IResult<&str, Section> {
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };
    let children = if source.trim().is_empty() {
        None
    } else {
        Some(Block::RawContent {
            text: Some(source.trim().to_string()),
        })
    };
    Ok((
        source,
        Section::ScriptSection {
            attributes,
            children,
        },
    ))
}

#[cfg(test)]
mod test {
    use crate::block::block::*;
    use crate::parse::parse;
    // use crate::section::attributes_for_section::*;
    use crate::section::section::*;
    use crate::wrapper::wrapper::*;

    #[test]
    fn basic_css() {
        let lines = vec!["-> script", "", "console.log(`here`)"].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::ScriptSection {
                attributes: None,
                children: Some(Block::RawContent {
                    text: Some("console.log(`here`)".to_string()),
                }),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}
