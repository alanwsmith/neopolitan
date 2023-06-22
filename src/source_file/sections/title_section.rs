use crate::source_file::source_file::SourceFile;
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
                    .unwrap()
                    .to_string(),
            ),
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::source_file::source_file::SourceFile;

    #[test]
    pub fn test_title() {
        let mut sf = SourceFile::new();
        let lines = vec!["-> title", "", "Delta Hotel"];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content(),
            Some(String::from(r#"<h1 class="neo-title">Delta Hotel</h1>"#))
        );
    }
}

