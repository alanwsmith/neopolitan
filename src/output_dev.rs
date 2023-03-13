#![allow(warnings)]
use crate::page::Page;
use crate::section::Section::*;

pub fn output_dev(source: Page) -> String {
    let mut output = "".to_string();
    for child in source.children {
        match child {
            PLAINTEXT { value } => {}
            UNORDERED_LIST_ITEM {
                attributes,
                children,
            } => {}
            UNORDERED_LIST {
                attributes,
                children,
            } => {}
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
                            output.push_str("\n");
                        }
                        _ => {}
                    }
                }
            }
            P {
                attributes,
                children,
            } => {
                for title_child in children {
                    match title_child {
                        PLAINTEXT { value } => {
                            output.push_str(r#"<p>"#);
                            output.push_str(value.as_str());
                            output.push_str(r#"</p>"#);
                            output.push_str("\n");
                        }
                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }

    dbg!(&output);
    r#"<h1 class="title">This Is A Title</h1>
<p>First paragraph</p>
<p>Second paragraph</p>"#
        .to_string();

    output
}
