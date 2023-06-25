// use crate::source_file::SourceFile;
//
//
use crate::source_file::block::block;
use minijinja::context;
use minijinja::Environment;
use minijinja::Source;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

// impl SourceFile {
pub fn title_section(source: &str) -> IResult<&str, Option<String>> {
    let (_, b) = many_till(
        alt((tuple((take_until("\n\n"), tag("\n\n"))).map(|x| x.0), rest)),
        eof,
    )(source)?;
    let main_title = b.0.clone().into_iter().next().unwrap();
    let paragraphs: Vec<_> =
        b.0.clone()
            .into_iter()
            .skip(1)
            .map(|x| block(x).unwrap().1)
            .collect();
    // dbg!(&paragraphs);
    let mut env = Environment::new();
    env.set_source(Source::from_path("./templates"));
    let wrapper = env.get_template("sections/title.j2").unwrap();
    Ok((
        "",
        Some(
            wrapper
                .render(context!(
                main_title => main_title,
                paragraphs => paragraphs,
                    ))
                .unwrap(),
        ),
    ))
}
// }

#[cfg(test)]
mod test {
    use super::*;
    // use crate::source_file::SourceFile;
    use rstest::rstest;
    // use std::path::PathBuf;

    #[rstest]
    #[case(vec!["Echo Whiskey"].join(""), 
        Ok(("", Some(format!(r#"<hgroup><h1>Echo Whiskey</h1></hgroup>"#)))
    ))]
    #[case(vec!["Echo Whiskey", "", "Delta Echo"].join("\n"), 
        Ok(("", Some(format!(r#"<hgroup><h1>Echo Whiskey</h1><p>Delta Echo</p></hgroup>"#)))
    ))]
    #[case(vec!["Foxtrot Hotel", "", "Tango Whiskey", "", "Delta Alfa"].join("\n"), 
        Ok(("", Some(format!(r#"<hgroup><h1>Foxtrot Hotel</h1><p>Tango Whiskey</p><p>Delta Alfa</p></hgroup>"#)))
    ))]
    #[case(vec!["Bravo Charlie", "", "Alfa <<Delta|strong>> Sierra"].join("\n"), 
        Ok(("", Some(format!(r#"<hgroup><h1>Bravo Charlie</h1><p>Alfa <strong>Delta</strong> Sierra</p></hgroup>"#)))
    ))]

    pub fn run_tests_for_title(
        #[case] input: String,
        #[case] expected: IResult<&str, Option<String>>,
    ) {
        assert_eq!(expected, title_section(input.as_str()));
    }

    //
}
