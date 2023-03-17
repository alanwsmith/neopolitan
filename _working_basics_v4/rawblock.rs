#[derive(Debug, PartialEq)]
pub enum RawBlock {
    Attributes { text: String },
    Blurb { text: String },
    Categories { text: String },
    Div { text: String },
    H1 { text: String },
    H2 { text: String },
    H3 { text: String },
    H4 { text: String },
    H5 { text: String },
    H6 { text: String },
    Code { text: String },
    Error { text: String },
    P { text: String },
    Title { text: String },
}
