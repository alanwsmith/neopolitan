use crate::section::attributes_for_section::*;
use crate::section::section::Section;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::multi::many0;
// use nom::sequence::preceded;
// use nom::error;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn categories(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (remainder, att_capture) = many0(
        tuple((tag(">> "), not_line_ending::<&str, _>, line_ending)).map(|x| {
            SectionAttribute::Category {
                category: Some(x.1.to_string()),
            }
        }),
    )(source)?;
    // dbg!(&att_capture);
    Ok((
        remainder,
        Section::CategoriesSection {
            categories: Some(att_capture),
        },
    ))
}

#[cfg(test)]
mod test {
    use crate::parse::parse;
    use crate::section::attributes_for_section::*;
    use crate::section::section::*;
    use crate::wrapper::wrapper::*;

    #[test]
    fn categories_implementation() {
        let lines = vec!["-> categories", "", ">> Test", ">> Reference", ""].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::CategoriesSection {
                categories: Some(vec![
                    SectionAttribute::Category {
                        category: Some("Test".to_string()),
                    },
                    SectionAttribute::Category {
                        category: Some("Reference".to_string()),
                    },
                ]),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}
