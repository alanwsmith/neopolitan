#![allow(warnings)]
use crate::builder::Builder;
use minijinja::{
    context,
    Environment,
};
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_until1;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::IResult;
use std::fs;
use std::include_str;
use std::path::PathBuf;

// impl Builder {
//     pub fn find_tag(
//         i: &str,
//     ) -> IResult<&str, &str> {
//         Ok(("", ""))
//     }
// }

impl Builder {
    pub fn output(
        &self, template_path: PathBuf,
    ) -> String {
        let result: IResult<&str, &str> =
            tag("-> ")(
                &self.source.unwrap().as_str(),
            );

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

impl Builder {
    pub fn output_dev(
        &self, template_path: PathBuf,
    ) -> IResult<String, &str> {
        dbg!("-----------------");
        // grab the first tag directly.
        // other tags will come in from take_while
        // that looks for preceeing newlines that
        // aren't present at the start of the file.
        // This is hard coded to have the title
        // as the first tag. TBD on when it'll
        // be worth the effort to provide for
        // other tags.
        let result: IResult<&str, &str> =
            tag("-> ")(
                &self.source.unwrap().as_str(),
            );

        let result: IResult<&str, &str> =
            alpha1(result.unwrap().0);

        let result: IResult<&str, &str> =
            multispace1(result.unwrap().0);

        let result: IResult<&str, &str> =
            not_line_ending(result.unwrap().0);

        let title = result.as_ref().unwrap().1;
        // dbg!(title);

        dbg!(result.as_ref().unwrap().0);

        // Get the next tag.
        let result: IResult<&str, &str> =
            take_until("\n-> ")(result.unwrap().0);

        dbg!(result.as_ref().unwrap().1);

        match result {
            Ok(value) => {
                dbg!("HERE");
            }
            Err(e) => {
                dbg!("THERE");
            }
        }

        // dbg!(result.unwrap().0);

        // dbg!(result.unwrap().1

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

        let the_output = format!(
            "{}",
            tmpl.render(context!(title => title))
                .unwrap()
        );

        Ok((the_output, ""))
    }
}
