use scraper::{Html, Selector};
use std::fs;

#[test]
fn home_page_has_title() {
    neopolitan::clear_output_directory::clear_output_directory();
    neopolitan::build_site::build_site();
    let source =
        fs::read_to_string("site/index.html").unwrap();
    let doc = Html::parse_document(source.as_str());
    let selector = Selector::parse("title").unwrap();
    let element = doc.select(&selector).next().unwrap();
    assert_eq!(
        "Neopolitan Test Site",
        element.inner_html()
    );
}
