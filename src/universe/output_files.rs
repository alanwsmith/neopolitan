use crate::universe::universe::Universe;

impl Universe<'_> {
    pub fn output_files(&self) {
        println!("Outputting files");
        let mut counter: u32 = 0;
        for (source_path, _source_file) in self.content_files.iter() {
            self.output_file(source_path.to_path_buf());
            counter += 1;
            if counter % 100 == 0 {
                println!("Count: {}", counter);
            }
        }
        println!("Output finished");
        println!("Total Files: {}", counter);
    }
}
