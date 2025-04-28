use nom::IResult;
use crate::parsers::neotags::NeoTag;
use nom::multi::many0;
use crate::parsers::global_attribute::global_attribute;

pub fn abbr_tag(source: &str) -> IResult<&str, NeoTag> {
    let(tmp_source, asdf) = many0(global_attribute)(source)?;
// dbg!(&source);
Ok((source, NeoTag::Empty))
}