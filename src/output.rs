#![allow(warnings)]
use crate::builder::Builder;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::IResult;

impl Builder {
    pub fn output(&self) -> String {
        let result: IResult<&str, &str> =
            tag("-> ")(&self.source.as_str());

        let result: IResult<&str, &str> =
            alpha1(result.unwrap().0);

        let result: IResult<&str, &str> =
            multispace1(result.unwrap().0);

        let result: IResult<&str, &str> =
            not_line_ending(result.unwrap().0);
        format!("<h1>{}</h1>", result.unwrap().1)
    }
}
