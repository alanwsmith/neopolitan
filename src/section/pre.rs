use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::character::complete::multispace0;
use nom::IResult;

pub fn pre(source: &str) -> IResult<&str, Section> {
    let (remainder, attributes) = section_attributes(source)?;
    let (remainder, _) = multispace0(remainder)?;
    let escaped_text = html_escape::encode_text(remainder).to_string();
    Ok((
        remainder,
        Section::PreSection {
            attributes,
            raw: Some(escaped_text.to_string()),
        },
    ))
}

#[cfg(test)]
mod test {

    use crate::section::pre::*;

    #[test]
    pub fn basic_pre() {
        let source = ["Bring your best compass", "Cap the jar"]
            .join("\n")
            .to_string();
        let expected = Section::PreSection {
            attributes: None,
            raw: Some("Bring your best compass\nCap the jar".to_string()),
        };
        let results = pre(&source).unwrap().1;
        assert_eq!(expected, results);
    }

    #[test]
    pub fn pre_with_attributes() {
        let source = [">> class: highlight", "The canoe", "is made of oak"]
            .join("\n")
            .to_string();
        let expected = Section::PreSection {
            attributes: Some(vec![SectionAttribute::Attribute {
                key: Some("class".to_string()),
                value: Some("highlight".to_string()),
            }]),
            raw: Some("The canoe\nis made of oak".to_string()),
        };
        let results = pre(&source).unwrap().1;
        assert_eq!(expected, results);
    }
}
