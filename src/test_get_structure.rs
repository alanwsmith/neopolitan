/////////////////////////////////////////////
// This is the basic integration test
// location. GOAL: It has a few samples
// that demonstrate the core features
// of the format. Detailed variatsions are
// checked via unit tests.
/////////////////////////////////////////////

use crate::chunk::Chunk;
use crate::get_structure::get_structure;
use crate::page::Page;
use crate::section::Section;
use std::collections::HashMap;

#[test]
fn test_get_structure_001() {
    let source = vec!["-> TITLE", "", "Alfa Bravo"].join("\n");
    let expected = Page {
        attributes: HashMap::new(),
        children: vec![Section::TITLE {
            children: vec![Chunk::H1 {
                attributes: HashMap::from([("class".to_string(), "title".to_string())]),
                children: vec![Chunk::Text {
                    value: "Alfa Bravo".to_string(),
                }],
            }],
        }],
    };
    let result = get_structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

// #[test]
// fn test_get_structure_002() {
//     let source = vec![
//         "-> TITLE",
//         ">> id: main",
//         "",
//         "Alfa Bravo",
//         "",
//         "The quick brown fox",
//         "and the lazy dog",
//         "",
//         "-> P",
//         "",
//         "They took the axe and the",
//         "saw to the forest.",
//         "",
//         "The bark of the pine tree.",
//         "",
//         "-> P",
//         ">> class: main",
//         "",
//         "A chink in the wall.",
//         "",
//     ]
//     .join("\n");
//     let expected = Page {
//         attributes: HashMap::new(),
//         children: vec![
//             Section::TITLE {
//                 children: vec![
//                     Chunk::H1 {
//                         attributes: HashMap::from([
//                             ("id".to_string(), "main".to_string()),
//                             ("class".to_string(), "title".to_string()),
//                         ]),
//                         children: vec![Chunk::Text {
//                             value: "Alfa Bravo".to_string(),
//                         }],
//                     },
//                     Chunk::P {
//                         attributes: HashMap::from([]),
//                         children: vec![Chunk::Text {
//                             value: "The quick brown fox\nand the lazy dog".to_string(),
//                         }],
//                     },
//                 ],
//             },
//             Section::P {
//                 children: vec![
//                     Chunk::P {
//                         attributes: HashMap::from([]),
//                         children: vec![Chunk::Text {
//                             value: "They took the axe and the\nsaw to the forest.".to_string(),
//                         }],
//                     },
//                     Chunk::P {
//                         attributes: HashMap::from([]),
//                         children: vec![Chunk::Text {
//                             value: "The bark of the pine tree.".to_string(),
//                         }],
//                     },
//                 ],
//             },
//             Section::P {
//                 children: vec![Chunk::P {
//                     attributes: HashMap::from([("class".to_string(), "main".to_string())]),
//                     children: vec![Chunk::Text {
//                         value: "The chink in the wall.".to_string(),
//                     }],
//                 }],
//             },
//         ],
//     };
//     let result = get_structure(source.as_str()).unwrap().1;
//     assert_eq!(expected, result);
// }
