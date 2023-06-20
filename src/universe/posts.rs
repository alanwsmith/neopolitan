use crate::universe::universe::Universe;

impl Universe<'_> {
    pub fn posts(&self) -> Vec<(String, String)> {
        let mut post_list = vec![];
        // dbg!(&self.content_files);
        // &self.content_files.iter().foreach(|content_file| dbg!(content_file); ())

        post_list.push((
            String::from("/posts/test-post-1/index.html"),
            String::from("Hotel India Post"),
        ));

        post_list
    }
}
