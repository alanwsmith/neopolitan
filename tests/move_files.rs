use std::path::PathBuf;

#[test]
fn home_page_has_title() {
    neopolitan::clear_output_directory::clear_output_directory();
    neopolitan::build_site::build_site();
    assert_eq!(true, PathBuf::from("site/posts/alfa/index.html").exists());
}
