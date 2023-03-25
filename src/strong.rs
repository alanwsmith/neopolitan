use crate::chunk::Chunk;
use crate::split::split;
use nom::IResult;

pub fn strong<'a>(
    source: &'a str,
    _raw_attributes: &'a str,
    remainder: &'a str,
) -> IResult<&'a str, Chunk> {
    let (_, stuff) = split(source, "|")?;
    dbg!(&stuff);
    let response = Chunk::Strong {
        attributes: None,
        value: Some(stuff[0].to_string()),
    };

    //     Target::Link { pretext } => {
    //         response.push(Chunk::Text {
    //             attributes: None,
    //             value: Some(pretext.to_string()),
    //         });
    //         let (source, _) = tag("<<")(payload.1)?;
    //         let (remainder, stuff) = take_until(">>")(source)?;
    //         let (_, stuff) = split(stuff, "|")?;
    //         let value = Some(stuff[2].to_string());
    //         let url = Some(stuff[1].to_string());
    //         let (remainder, _) = tag(">>")(remainder)?;
    //         if stuff.len() > 3 {
    //             let (_, attributes) = parse_text_attributes(stuff[3])?;
    //             response.push(Chunk::Link {
    //                 value,
    //                 url,
    //                 attributes,
    //             });
    //             Ok((remainder, response))
    //         } else {
    //             response.push(Chunk::Link {
    //                 value,
    //                 url,
    //                 attributes: None,
    //             });
    //             Ok((remainder, response))
    //         }
    //     }
    //     Target::Strong { pretext } => {
    //         response.push(Chunk::Text {
    //             attributes: None,
    Ok((remainder, response))
}
