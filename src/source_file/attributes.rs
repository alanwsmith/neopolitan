#![allow(unused_imports)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

pub fn attributes(
    source: &str,
) -> IResult<&str, Vec<(&str, &str)>> {
    let (a, b) = opt(tuple((
        tag(">> "),
        separated_list0(
            tag(">>"),
            alt((take_until(">>"), rest)),
        ),
    )))(source)?;

    let delta = b
        .unwrap()
        .1
        .into_iter()
        .map(|x| x.split(":").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let echo = delta
        .iter()
        .map(|r| (r[0].trim(), r[1].trim()))
        .collect::<Vec<(&str, &str)>>();
    Ok((a, echo))

    //
}

#[cfg(test)]

mod test {

    use super::*;

    // #[test]
    // pub fn basic_attr_test() {
    //     let lines = [">> class: echo"].join("\n");
    //     assert_eq!(
    //         attributes(lines.as_str()),
    //         Ok(("", vec![("class", "echo")]))
    //     );
    // }

    // #[test]
    // pub fn with_two_attirbutes() {
    //     let lines =
    //         [">> class: foxtrot >> id: charlie"].join("\n");
    //     assert_eq!(
    //         attributes(lines.as_str()),
    //         Ok((
    //             "",
    //             vec![
    //                 ("class", "foxtrot"),
    //                 ("id", "charlie")
    //             ]
    //         ))
    //     );
    // }

    //
}
