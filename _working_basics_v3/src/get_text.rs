use crate::section::Section;
use nom::IResult;

pub fn get_text(source: &str) -> IResult<&str, Vec<Section>> {
    Ok((
        source,
        vec![Section::PLAINTEXT {
            value: source.to_string(),
        }],
    ))
}
