use std::path::PathBuf;

#[test]
#[ignore]
fn index_file_exists() {
    neopolitan::clear_output_directory::clear_output_directory();
    assert_eq!(
        false,
        PathBuf::from("./site/index.html").exists()
    );
    neopolitan::build_site::build_site();
    assert_eq!(
        true,
        PathBuf::from("./site/index.html").exists()
    );
}
