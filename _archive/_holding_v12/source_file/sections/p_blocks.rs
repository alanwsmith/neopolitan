use crate::source_file::block::block;
use itertools::Itertools;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::multi::separated_list0;
use nom::IResult;

// this is what can be used by other
// things to return blocks of paragraphs

pub fn p_blocks(
    source: &str,
) -> IResult<&str, Vec<String>> {
    let (_, captured) = separated_list0(
        line_ending,
        not_line_ending,
    )(source.trim())?;
    let text_lines = captured
        .into_iter()
        .map(|s| s.to_string())
        .coalesce(|x, y| {
            if !x.is_empty() && !y.is_empty() {
                Ok(format!("{x} {y}"))
            } else {
                Err((x, y))
            }
        })
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .into_iter()
        .map(|s| format!("<p>{}</p>", block(&s).unwrap().1))
        .collect::<Vec<String>>();
    // dbg!(&text_lines);
    Ok(("", text_lines))
}

#[cfg(test)]

mod test {

    // use super::*;

    //     #[test]
    //     pub fn p_blocks() {
    //         let lines = [
    //             "Hoist it up",
    //             "and <<take|strong>> it away",
    //             "",
    //             "Pluck the rose",
    //             "",
    //             "Tear a thin",
    //             "sheet from",
    //             "the yellow pad",
    //         ];
    //         assert_eq!(
    //             p_blocks(lines.join("\n").as_str()),
    //             Ok((
    //                 "",
    //                 vec![
    //                     r#"<p>Hoist it up and <strong>take</strong> it away</p>"#.to_string(),
    //                     r#"<p>Pluck the rose</p>"#.to_string(),
    //                     r#"<p>Tear a thin sheet from the yellow pad</p>"#.to_string(),
    //                 ]
    //             ))
    //         );
    //     }
}
