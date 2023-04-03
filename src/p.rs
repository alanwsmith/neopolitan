use crate::block::Block;
use crate::content::*;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace1;
use nom::IResult;

pub fn p(source: &str) -> IResult<&str, Block> {
    let (source, captured) = take_until("\n\n")(source)?;
    let (source, _) = multispace1(source)?;
    let (_, b) = content(captured)?;
    Ok((
        source,
        Block::P {
            attributes: None,
            children: Some(b),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Block;
    use crate::content::Content;
    #[test]
    fn basic_title_response() {
        let source = "alfa bravo\n\n";
        let expected = Ok((
            "",
            Block::P {
                attributes: None,
                children: Some(vec![Content::Text {
                    text: Some("alfa bravo".to_string()),
                }]),
            },
        ));
        let result = p(source);
        assert_eq!(expected, result);
    }
}
