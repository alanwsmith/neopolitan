use std::path::PathBuf;
// use walkdir::WalkDir;

#[derive(Debug)]
pub struct Files {
    pub content_dir: Option<PathBuf>,
    // pub files: Vec<SourceFile>,
}

// impl Files {
//     pub fn posts(&self) -> Vec<(PathBuf, String)> {
//         let mut post_data = vec![];
//         self.files.iter().for_each(|file| {
//             let mut url_path = PathBuf::from("/");
//             url_path.push(file.raw_path.clone().unwrap());
//             url_path.set_extension("html");
//             post_data.push((url_path, String::from("Title Goes Here")));
//             ()
//         });
//         post_data
//     }
// }

// impl Files {
//     pub fn load_files(&mut self) -> Result<(), Error> {
//         for entry in WalkDir::new(&self.content_dir.as_ref().unwrap()).into_iter() {
//             let p = entry?.path().to_path_buf();
//             if let Some(ext) = p.extension() {
//                 if ext == "neo" {
//                     let mut sf = SourceFile::new();
//                     sf.raw = Some(fs::read_to_string(&p.to_str().unwrap()).unwrap());
//                     sf.raw_path = Some(
//                         p.strip_prefix(&self.content_dir.as_ref().unwrap())
//                             .unwrap()
//                             .to_path_buf(),
//                     );
//                     let _ = &self.files.push(sf);
//                 }
//             }
//         }
//         Ok(())
//     }
// }
