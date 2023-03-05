#![allow(warnings)]
use crate::builder::Builder;
use minijinja::{
    context,
    Environment,
};
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::IResult;
use std::include_str;

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

        let mut env = Environment::new();

        env.add_template(
            "title",
            include_str!(
                "../test_sets/full/1/post.html"
            ),
        );

        // env.add_template(
        //     "title",
        //     "<h1>{{ title }}</h1>

        // ",
        //         )
        //         .unwrap();

        let tmpl =
            env.get_template("title").unwrap();

        format!(
            "{}",
            tmpl.render(
                context!(title => result.unwrap().1)
            )
            .unwrap()
        )
    }
}
