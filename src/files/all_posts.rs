use crate::files::Files;

impl Files {
    pub fn all_posts(&self) -> Vec<(String, String)> {
        let files = self
            .files
            .iter()
            .filter(|file| {
                file.content_type()
                    .is_some_and(|ct| ct == "post")
            })
            .map(|file| {
                (
                    file.title().unwrap(),
                    file.url()
                        .unwrap()
                        .display()
                        .to_string(),
                )
            })
            .collect();
        files
    }
}

#[cfg(test)]
mod test {
    use crate::files::Files;
    use crate::source_file::SourceFile;
    use std::path::PathBuf;

    #[test]
    pub fn test_posts_basic() {
        let mut content = Files { files: vec![] };
        let lines = vec![
            "-> title",
            "",
            "Alfa Bravooo",
            "",
            "-> attributes",
            ">> type: post",
            "",
        ];
        let sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(
                "some/path/index.neo",
            ),
        };

        content.files.push(sf);
        assert_eq!(
            content.all_posts(),
            vec![(
                String::from("Alfa Bravooo"),
                String::from("/some/path/index.html")
            )],
        );
    }
}
