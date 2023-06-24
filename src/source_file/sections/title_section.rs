use crate::source_file::SourceFile;
use minijinja::context;
use minijinja::Environment;
use minijinja::Source;
use nom::IResult;

impl SourceFile {
    pub fn title_section(&self, source: &str) -> IResult<&str, Option<String>> {
        let mut env = Environment::new();
        env.set_source(Source::from_path("./templates"));
        let wrapper = env.get_template("sections/title.j2").unwrap();
        Ok((
            "",
            Some(
                wrapper
                    .render(context!(
                        title => String::from(source.trim()),
                    ))
                    .unwrap(),
            ),
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::source_file::SourceFile;
    use rstest::rstest;
    use std::path::PathBuf;

    #[rstest]
    #[case(vec!["-> title", "", "Echo Whiskey"], 
 Some(format!(r#"<h1 class="neo-title">Echo Whiskey</h1>"#)
    ))]
    pub fn run_tests(#[case] input: Vec<&str>, #[case] expected: Option<String>) {
        let sf = SourceFile {
            source_data: input.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(expected, sf.content());
    }

    // #[test]
    // pub fn test_title() {
    //     let lines = vec!["-> title", "", "Delta Hotel"];
    //     let sf = SourceFile {
    //         source_data: lines.join("\n"),
    //         source_path: PathBuf::from(""),
    //     };
    //     assert_eq!(
    //         sf.content(),
    //         Some(String::from(r#"<h1 class="neo-title">Delta Hotel</h1>"#))
    //     );
    // }
}
