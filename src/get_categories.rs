use nom::IResult;

pub fn get_categories(source: &str) -> IResult<&str, Vec<String>> {
    let categories: Vec<String> = vec!["Rust".to_string(), "Test".to_string()];
    Ok((source, categories))
}
