use crate::source_file::source_file::SourceFile;
use crate::universe::universe::Universe;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
// use std::path::PathBuf;

impl Universe<'_> {
    pub fn get_categories<'a>(&'a mut self, source_file: &'a SourceFile) -> IResult<&str, &str> {
        let (remainder1, _) =
            take_until("-> categories")(source_file.raw.as_ref().unwrap().as_str())?;
        let (remainder2, _) = tuple((tag("-> categories"), multispace0))(remainder1)?;
        let (_, captured3) = many0(tuple((tag(">> "), not_line_ending, line_ending)))(remainder2)?;
        for category in captured3.iter() {
            let here_now_is_clone = source_file.clone();
            if self
                .categories
                .contains_key(&String::from(category.1.trim()))
            {
                self.categories
                    .get_mut(&String::from(category.1.trim()))
                    .unwrap()
                    .push(here_now_is_clone.output_path().unwrap());
            } else {
                self.categories.insert(
                    String::from(category.1.trim()),
                    vec![here_now_is_clone.output_path().unwrap()],
                );
            }
        }
        Ok(("", ""))
    }
}

#[cfg(test)]
mod test {
    use crate::source_file::source_file::SourceFile;
    use crate::universe::universe::Universe;
    use std::path::PathBuf;

    #[test]
    pub fn basic_category_test() {
        let mut u = Universe::new();
        let mut sf = SourceFile::new();
        let lines = [
            "-> title",
            "Alfa Bravo",
            "",
            "-> categories",
            ">> Posts",
            "",
        ];
        sf.raw = Some(lines.join("\n").to_string());
        let file_path = PathBuf::from("some/path.neo");
        sf.raw_path = Some(PathBuf::from("some/path.neo"));
        u.content_files.insert(file_path, sf.clone());
        let _get_categories_status = u.get_categories(&sf);
        assert_eq!(true, u.categories.contains_key("Posts"));
        let expected_vec = vec![PathBuf::from("some/path.html")];
        assert_eq!(
            &expected_vec,
            u.categories.get(&String::from("Posts")).unwrap()
        );
    }

    #[test]
    pub fn two_file_test() {
        let mut u = Universe::new();
        let mut alfa_file = SourceFile::new();
        let alfa_lines = ["-> categories", ">> Widget", ""];
        alfa_file.raw = Some(alfa_lines.join("\n").to_string());
        let alfa_path = PathBuf::from("some/alfa.neo");
        alfa_file.raw_path = Some(PathBuf::from("some/alfa.neo"));
        u.content_files.insert(alfa_path, alfa_file.clone());
        let _get_categories_status = u.get_categories(&alfa_file);
        let mut bravo_file = SourceFile::new();
        let bravo_lines = ["-> categories", ">> Widget", ""];
        bravo_file.raw = Some(bravo_lines.join("\n").to_string());
        let bravo_path = PathBuf::from("some/bravo.neo");
        bravo_file.raw_path = Some(PathBuf::from("some/bravo.neo"));
        u.content_files.insert(bravo_path, bravo_file.clone());
        let _get_categories_status = u.get_categories(&bravo_file);
        let expected_vec = vec![
            PathBuf::from("some/alfa.html"),
            PathBuf::from("some/bravo.html"),
        ];
        assert_eq!(
            &expected_vec,
            u.categories.get(&String::from("Widget")).unwrap()
        );
    }

    #[test]
    pub fn multiple_categories_test() {
        let mut u = Universe::new();
        let mut sf = SourceFile::new();
        let lines = ["-> categories", ">> Delta", ">> Echo", ""];
        sf.raw = Some(lines.join("\n").to_string());
        let file_path = PathBuf::from("some/path.neo");
        sf.raw_path = Some(PathBuf::from("some/path.neo"));
        u.content_files.insert(file_path, sf.clone());
        let _get_categories_status = u.get_categories(&sf);
        let expected_vec = vec![PathBuf::from("some/path.html")];
        assert_eq!(
            &expected_vec,
            u.categories.get(&String::from("Echo")).unwrap()
        );
    }

    #[test]
    pub fn white_space_after_category_test() {
        let mut u = Universe::new();
        let mut sf = SourceFile::new();
        let lines = ["-> categories", ">> Echo Foxtrot ", ""];
        sf.raw = Some(lines.join("\n").to_string());
        let file_path = PathBuf::from("some/path.neo");
        sf.raw_path = Some(PathBuf::from("some/path.neo"));
        u.content_files.insert(file_path, sf.clone());
        let _get_categories_status = u.get_categories(&sf);
        assert_eq!(true, u.categories.contains_key("Echo Foxtrot"));
        let expected_vec = vec![PathBuf::from("some/path.html")];
        assert_eq!(
            &expected_vec,
            u.categories.get(&String::from("Echo Foxtrot")).unwrap()
        );
    }
}
