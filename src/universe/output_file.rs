use crate::universe::universe::Universe;
use minijinja::context;
use std::fs;
use std::path::PathBuf;

impl Universe<'_> {
    pub fn output_file(&self, path: PathBuf) {
        if let Some(source_file) = self.content_files.get(&path) {
            let mut full_path = self.output_root.clone().unwrap();
            full_path.push(source_file.output_path().unwrap());
            let dir_path = full_path.parent();
            fs::create_dir_all(dir_path.unwrap()).unwrap();
            let mut template_file = source_file.file_type().unwrap().1.unwrap();
            template_file.push_str(".j2");
            if let Some(_) = source_file.output(self) {
                let wrapper = self
                    .env
                    .as_ref()
                    .unwrap()
                    .get_template(&template_file)
                    .unwrap();
                let out = wrapper
                    .render(context!(
                    content =>
                     source_file.output(self).unwrap()
                        ))
                    .unwrap()
                    .to_string();
                fs::write(full_path, out).unwrap();
            }
        }
    }
}
