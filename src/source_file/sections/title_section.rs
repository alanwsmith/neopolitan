#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::source_file::attributes::attributes;
use crate::source_file::block::block;
use crate::source_file::paragraphs::paragraphs;
use minijinja::context;
use minijinja::Environment;
use minijinja::Source;
use nom::IResult;

pub fn title_section(
    source: &str,
) -> IResult<&str, Option<String>> {
    let (_, b) = paragraphs(source)?;
    dbg!(&b);

    let input = b.clone().into_iter().next().unwrap();
    dbg!(&input);

    let jejej = b.clone().into_iter().next().unwrap();

    // let (_a, _c) = attributes(input.as_str())?;
    let (a, c) = attributes(jejej.as_str()).unwrap();

    //let (_a, _c) = attributes(input.as_str())?;
    // let whatever = b.clone()[0];
    // let whatever = b[0];
    // let (a, c) = attributes(whatever.as_str())?;
    dbg!(&a);
    dbg!(&c);

    // let (a, b) = opt(
    //     tuple((tag(">>"), multispace1, rest)),
    // )(b)
    // b.clone().into_iter().next().unwrap();

    let main_title = b.clone().into_iter().next().unwrap();
    dbg!(&main_title);

    let paragraphs: Vec<_> = b
        .clone()
        .into_iter()
        .skip(1)
        .map(|x| block(&x).unwrap().1)
        .collect();
    dbg!(&paragraphs);
    let mut env = Environment::new();
    env.set_source(Source::from_path("./templates"));
    let wrapper =
        env.get_template("sections/title.j2").unwrap();

    Ok(("", None))

    // Ok((
    //     "",
    //     Some(
    //         wrapper
    //             .render(context!(
    //             main_title => main_title,
    //             paragraphs => paragraphs,
    //                 ))
    //             .unwrap(),
    //     ),
    // ))
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // #[case(vec!["Echo Whiskey"].join(""),
    //     Ok(("", Some(format!(r#"<hgroup><h1>Echo Whiskey</h1></hgroup>"#)))
    // ))]
    // #[case(vec!["Echo Whiskey", "", "Delta Echo"].join("\n"),
    //     Ok(("", Some(format!(r#"<hgroup><h1>Echo Whiskey</h1><p>Delta Echo</p></hgroup>"#)))
    // ))]
    // #[case(vec!["Foxtrot Hotel", "", "Tango Whiskey", "", "Delta Alfa"].join("\n"),
    //     Ok(("", Some(format!(r#"<hgroup><h1>Foxtrot Hotel</h1><p>Tango Whiskey</p><p>Delta Alfa</p></hgroup>"#)))
    // ))]
    // #[case(vec!["Bravo Charlie", "", "Alfa <<Delta|strong>> Sierra"].join("\n"),
    //     Ok(("", Some(format!(r#"<hgroup><h1>Bravo Charlie</h1><p>Alfa <strong>Delta</strong> Sierra</p></hgroup>"#)))
    // ))]
    #[case(vec![">> class: echo", "", "Foxtrot Tango", "", "Sierra Hotel"].join("\n"),
        Ok(("", Some(format!(r#"<hgroup class="echo"><h1>Foxtrot Tango</h1><p>Sierra Hotel</p></hgroup>"#)))
    ))]

    // #[case(vec![">> class: echo", "id: victor", "", "Foxtrot Tango", "", "Sierra Hotel"].join("\n"),
    //     Ok(("", Some(format!(r#"<hgroup class="echo"><h1>Foxtrot Tango</h1><p>Sierra Hotel</p></hgroup>"#)))
    // ))]

    pub fn solo_run_tests_for_title(
        #[case] input: String,
        #[case] expected: IResult<&str, Option<String>>,
    ) {
        assert_eq!(expected, title_section(input.as_str()));
    }
}
