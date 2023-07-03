pub fn remove_whitespace(source: Option<String>) -> Option<String> {
    let output: String = source
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    Some(output)
}
