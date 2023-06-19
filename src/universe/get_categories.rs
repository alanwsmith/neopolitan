use crate::source_file::source_file::SourceFile;
use crate::universe::universe::Universe;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

impl Universe<'_> {
    pub fn get_categories<'a>(&'a mut self, source_file: &'a SourceFile) -> IResult<&str, &str> {
        let (remainder1, captured1) =
            take_until("-> categories")(source_file.raw.as_ref().unwrap().as_str())?;
        dbg!(&captured1);
        dbg!(&remainder1);

        let (remainder2, _) = tuple((tag("-> categories"), multispace0))(remainder1)?;
        dbg!(&remainder2);

        let (_, captured3) = many0(tuple((tag(">> "), not_line_ending)))(remainder2)?;
        dbg!(&captured3);

        for category in captured3.iter() {
            dbg!(&category.1);
            let here_now_is_clone = source_file.clone();
            self.categories.insert(
                String::from(category.1),
                vec![here_now_is_clone.raw_path.unwrap()],
            );
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
        let expected_vec = vec![PathBuf::from("some/path.neo")];
        assert_eq!(
            &expected_vec,
            u.categories.get(&String::from("Posts")).unwrap()
        );
    }
}
