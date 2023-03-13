#![allow(unused_imports)]
#[cfg(test)]
mod tests {

    use neopolitan::content::Content;
    use neopolitan::output_dev::output_dev;
    use neopolitan::output_switch::output_switch;
    use neopolitan::page::Page;
    use neopolitan::section::Section;
    use std::collections::HashMap;

    #[test]
    fn test_output_002() {
        let expected = r#"<h1 class="title">This Is A Title</h1>
<p>First paragraph</p>
<p>Second paragraph</p>
"#;

        let source = Page {
            attributes: HashMap::new(),
            children: vec![
                Section::TITLE {
                    attributes: HashMap::new(),
                    children: vec![Content::PLAINTEXT {
                        value: "This Is A Title".to_string(),
                    }],
                },
                Section::P {
                    attributes: HashMap::new(),
                    children: vec![Content::PLAINTEXT {
                        value: "First paragraph".to_string(),
                    }],
                },
                Section::P {
                    attributes: HashMap::new(),
                    children: vec![Content::PLAINTEXT {
                        value: "Second paragraph".to_string(),
                    }],
                },
            ],
        };

        // let source = Page {
        //     attributes: HashMap::new(),
        //     children: vec![Section::TITLE {
        //         attributes: HashMap::new(),
        //         children: vec![Content::PLAINTEXT {
        //             value: "This Is A Title".to_string(),
        //         }],
        //     }],
        // };

        let result = output_switch(source);
        assert_eq!(expected, result);
    }
}
