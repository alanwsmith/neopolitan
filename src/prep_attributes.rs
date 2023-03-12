use crate::get_attribute::*;
use crate::parse::Section;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;
use std::collections::HashMap;

pub fn prep_attributes(source: &str) -> IResult<&str, Section> {
    let (source, pairs) = many_till(get_attribute, eof)(source)?;
    let mut attrs: HashMap<String, String> = HashMap::new();
    for pair in pairs.0.iter() {
        attrs.insert(pair.0.to_string(), pair.1.to_string());
    }
    let the_attrs = Section::ATTRIBUTES { raw: attrs };
    Ok((source, the_attrs))
}
