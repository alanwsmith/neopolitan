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
use std::fs;
use std::include_str;
use std::path::PathBuf;

impl Builder {
    pub fn output(
        &self, template_path: PathBuf,
    ) -> String {
        let result: IResult<&str, &str> =
            tag("-> ")(&self.source.as_str());

        let result: IResult<&str, &str> =
            alpha1(result.unwrap().0);

        let result: IResult<&str, &str> =
            multispace1(result.unwrap().0);

        let result: IResult<&str, &str> =
            not_line_ending(result.unwrap().0);

        let mut env = Environment::new();

        let template_stuff =
            fs::read_to_string(template_path)
                .unwrap();

        env.add_template(
            "template",
            template_stuff.as_str(),
        );

        let tmpl =
            env.get_template("template").unwrap();

        format!(
            "{}",
            tmpl.render(
                context!(title => result.unwrap().1)
            )
            .unwrap()
        )
    }
}
