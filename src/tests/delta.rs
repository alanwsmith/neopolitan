use crate::attribute::*;
use crate::block::block::*;
use crate::content::content::*;
use crate::parse::parse;
use crate::section::section::*;
use crate::wrapper::wrapper::*;

#[ignore]
#[test]
fn delta() {
    let lines = vec![
        "-> p",
        "",
        "alfa <<b|bravo>> charlie",
        "<<b|delta>> <<b|echo>> <<b|foxtrot>>",
        "<<b|golf|class: advanced>>",
        "<<code|quick brown>>",
        "<<code|slow fox|lang: rust>>",
        "<<code|slow fox|rust>>",
        "<<em|some flower seeds>>",
        "<<i|red ink>>",
        "<<kbd|ctrl>>",
        "<<span|the worn floor>>",
        "<<strike|the worn floor>>",
        "<<strong|the worn floor>>",
        "<<sub|the worn floor>>",
        "<<sup|the worn floor>>",
        "<<u|the worn floor>>",
    ]
    .join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::Paragraphs {
            attributes: None,
            children: Some(vec![Block::P {
                children: Some(vec![
                    Content::Text {
                        text: Some("alfa".to_string()),
                    },
                    Content::Space,
                    Content::B {
                        attributes: None,
                        text: Some("bravo".to_string()),
                    },
                    Content::Space,
                    Content::Text {
                        text: Some("charlie".to_string()),
                    },
                    Content::Space,
                    Content::B {
                        attributes: None,
                        text: Some("delta".to_string()),
                    },
                    Content::Space,
                    Content::B {
                        attributes: None,
                        text: Some("echo".to_string()),
                    },
                    Content::Space,
                    Content::B {
                        attributes: None,
                        text: Some("foxtrot".to_string()),
                    },
                    Content::Space,
                    Content::B {
                        attributes: Some(vec![Attribute::Basic {
                            key: Some("class".to_string()),
                            value: Some("advanced".to_string()),
                        }]),
                        text: Some("golf".to_string()),
                    },
                    Content::Space,
                    Content::Code {
                        attributes: None,
                        text: Some("quick brown".to_string()),
                    },
                    Content::Space,
                    Content::Code {
                        attributes: Some(vec![Attribute::Basic {
                            key: Some("lang".to_string()),
                            value: Some("rust".to_string()),
                        }]),
                        text: Some("slow fox".to_string()),
                    },
                    Content::Space,
                    Content::Code {
                        attributes: Some(vec![Attribute::Basic {
                            key: Some("rust".to_string()),
                            value: None,
                        }]),
                        text: Some("slow fox".to_string()),
                    },
                    Content::Space,
                    Content::Em {
                        attributes: None,
                        text: Some("some flower seeds".to_string()),
                    },
                    Content::Space,
                    Content::I {
                        attributes: None,
                        text: Some("red ink".to_string()),
                    },
                    Content::Space,
                    Content::Kbd {
                        attributes: None,
                        text: Some("ctrl".to_string()),
                    },
                    Content::Space,
                    Content::Span {
                        attributes: None,
                        text: Some("the worn floor".to_string()),
                    },
                    Content::Space,
                    Content::Strike {
                        attributes: None,
                        text: Some("the worn floor".to_string()),
                    },
                    Content::Space,
                    Content::Strong {
                        attributes: None,
                        text: Some("the worn floor".to_string()),
                    },
                    Content::Space,
                    Content::Sub {
                        attributes: None,
                        text: Some("the worn floor".to_string()),
                    },
                    Content::Space,
                    Content::Sup {
                        attributes: None,
                        text: Some("the worn floor".to_string()),
                    },
                    Content::Space,
                    Content::U {
                        attributes: None,
                        text: Some("the worn floor".to_string()),
                    },
                ]),
            }]),
        }]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}
