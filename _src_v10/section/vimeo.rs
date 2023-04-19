use crate::section::section::*;
use crate::section::attributes_for_section::*;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

pub fn vimeo(source: &str) -> IResult<&str, Section> {
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };
    Ok((source, Section::VimeoSection { attributes }))
}


#[cfg(test)]
mod test {

    use crate::parse::parse;
use crate::section::section::*;
use crate::section::attributes_for_section::*;
use crate::wrapper::wrapper::*;

#[test]
fn mike() {
    let lines = vec!["-> vimeo", ">> 3817271", ""].join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![
            Section::VimeoSection {
                attributes: Some(vec![
                    (SectionAttribute::Attribute {
                        key: Some("3817271".to_string()),
                        value: None,
                    }),
                ]),
            },
        ]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}

}