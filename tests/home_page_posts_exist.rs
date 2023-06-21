use scraper::{Html, Selector};
use std::fs;

#[test]
fn home_page_posts_exist() {
    neopolitan::clear_output_directory::clear_output_directory();
    neopolitan::build_site::build_site();
    let source = fs::read_to_string("site/index.html").unwrap();
    let doc = Html::parse_document(source.as_str());
    let selector = Selector::parse("li").unwrap();
    // let element = doc.select(&selector).next().unwrap();
    let mut elements = doc.select(&selector);
    elements.next();
    let element = elements.next().unwrap();
    assert_eq!("Alfa Bravo", element.inner_html());
}
