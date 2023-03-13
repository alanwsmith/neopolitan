#![allow(warnings)]
use crate::content::Content::*;
use crate::page::Page;
use crate::section::Section::*;

pub fn output(source: Page) -> String {
    let mut output = "".to_string();
    for child in source.children {
        match child {
            TITLE {
                attributes,
                children,
            } => {
                for title_child in children {
                    match title_child {
                        PLAINTEXT { value } => {
                            output.push_str(r#"<h1 class="title">"#);
                            output.push_str(value.as_str());
                            output.push_str(r#"</h1>"#);
                        }
                    }
                }
            }
            P => {}
        }
    }
    output
}
