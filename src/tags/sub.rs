use crate::tags::Tag;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::sequence::delimited;
use nom::IResult;
use crate::tag_attrs::tag_attrs;
use nom::Parser;

pub fn sub(source: &str) -> IResult<&str, Tag> {
    let (source, text) =
        delimited(tag("<<"), is_not("|").map(|s: &str| s.to_string()), tag_no_case("|sub"))(source)?;
    let (source, attrs) = tag_attrs(source)?;
    let (source, _) = tag(">>")(source)?;
    Ok((
        source,
        Tag::Sub { text, attrs },
    ))
}
