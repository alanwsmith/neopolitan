// use crate::block::block::*;
use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::character::complete::multispace0;
// use nom::combinator::eof;
use crate::parse::parse::parse;
// use nom::multi::many_till;
use nom::IResult;
use crate::source_file::source_file::SourceFile;
use std::path::PathBuf;
use crate::universe::create_env::create_env;
use crate::universe::universe::Universe;

pub fn startdiv(source: &str) -> IResult<&str, Section> {

    // TODO: Figure out how to use the main universe
    // here instead of creating a new one so that
    // the paths are all defined in one place

    let templates_dir = "./site/templates";
    let assets_dir = "./site/assets/";
    let content_dir = "./site/content";
    let build_dir = "./site/build";
    let mut u = Universe::new();
    u.env = Some(create_env(templates_dir));
    u.assets_dir = Some(PathBuf::from(assets_dir));
    u.source_dir = Some(PathBuf::from(content_dir));
    u.dest_dir = Some(PathBuf::from(build_dir));
    let (remainder, attributes) = section_attributes(source)?;
    let (remainder, _) = multispace0(remainder)?;
    let mut internal_sf = SourceFile::new();
    internal_sf.parsed = parse(&remainder).unwrap().1;
    let output = internal_sf.output(&u);
    // dbg!(&output);
    Ok((
        remainder,
        Section::StartDivSection {
            attributes,
            html: output,
        },
    ))
}

#[cfg(test)]
mod test {

    use crate::parse::parse::*;
    use crate::source_file::source_file::*;
    use crate::tests::remove_whitespace::remove_whitespace;
    use crate::universe::create_env::create_env;
    use crate::universe::universe::Universe;

    #[test]
    pub fn startdiv_basic() {
        let source = [
            "-> startdiv",
            "",
            "-> h1",
            "",
            "Pluck the rose",
            "",
            "-> enddiv",
            "",

        ]
        .join("\n")
        .to_string();
        let expected = Some(
            vec![
                r#"<div>"#,
                r#"<h1>Pluck the rose</h1>"#,
                r#"</div>"#,
            ]
            .join("\n")
            .to_string(),
        );
        let mut u = Universe::new();
        u.env = Some(create_env("./site/templates"));
        let mut sf = SourceFile::new();
        sf.raw = Some(source);
        sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
        let output = sf.output(&u);
        assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    }

}
