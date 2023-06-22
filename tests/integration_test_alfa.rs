use scraper::{Html, Selector};
use std::fs;

#[test]
#[ignore]
fn integration_test_alfa() {
    neopolitan::clear_output_directory::clear_output_directory();
    neopolitan::build_site::build_site();
    let source = fs::read_to_string("site/posts/integration-test-alfa/index.html").unwrap();
    let doc = Html::parse_document(source.as_str());
    let selector = Selector::parse("p").unwrap();
    let mut elements = doc.select(&selector);
    assert_eq!(
        elements.next().unwrap().inner_html(),
        "This is the alfa integration test"
    );
    assert_eq!(
        elements.next().unwrap().inner_html(),
        "This is the second paragraph"
    );
    assert_eq!(
        elements.next().unwrap().inner_html(),
        "This is <strong>text with a strong tag</strong> applied"
    );
}
