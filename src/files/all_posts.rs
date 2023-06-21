use crate::files::files::Files;

impl Files {
    pub fn all_posts(&self) -> Vec<(String, String)> {
        let mut all_posts = vec![];
        self.files
            .iter()
            .for_each(|file| all_posts.push((file.title().unwrap(), String::from(""))));
        all_posts
    }
}

#[cfg(test)]
mod test {
    use crate::files::files::Files;
    use crate::source_file::source_file::SourceFile;

    #[test]
    pub fn test_posts_basic() {
        let mut content = Files::new();
        let mut sf = SourceFile::new();
        let lines = vec!["-> title", "", "Alfa Bravooo", "", "-> p"];
        sf.source_data = Some(lines.join("\n"));
        content.files.push(sf);
        assert_eq!(
            vec![(String::from("Alfa Bravooo"), String::from(""))],
            content.all_posts()
        );
    }
}
