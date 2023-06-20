use scraper::{Html, Selector};

#[test]
fn home_page_has_title() {
    let source = r#"
        <!DOCTYPE html>
        <html>
            <head><title>Alfa Bravo</title></head>
        </html>"#;
    let doc = Html::parse_document(source);
    let selector = Selector::parse("title").unwrap();
    let element = doc.select(&selector).next().unwrap();
    assert_eq!("Alfa Bravo", element.inner_html());
}
