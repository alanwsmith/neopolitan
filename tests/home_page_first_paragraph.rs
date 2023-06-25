use scraper::{Html, Selector};
use std::fs;

#[test]
fn home_page_first_paragraph() {
    neopolitan::clear_output_directory::clear_output_directory();
    neopolitan::build_site::build_site();
    let source = fs::read_to_string("site/index.html").unwrap();
    let doc = Html::parse_document(source.as_str());
    let selector = Selector::parse("p").unwrap();
    let element = doc.select(&selector).next().unwrap();
    assert_eq!(
        "This is a test run of the website builder",
        element.inner_html()
    );
}
