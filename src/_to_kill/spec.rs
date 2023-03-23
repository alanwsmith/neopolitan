use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum Spec {
    H1,
    InlineText,
    TitleSection { tmp: Option<String> },
    Wrapper,
    Page,
}
