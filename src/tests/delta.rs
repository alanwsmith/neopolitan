// use crate::block::block::*;
// use crate::content::content::*;
use crate::attribute::*;
use crate::block::block::*;
use crate::content::content::*;
use crate::parse::parse;
use crate::section::section::*;
use crate::wrapper::wrapper::*;

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
                ]),
            }]),
        }]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}

// Bring your best compass to the third class.
// A fresh start will work such wonders.
// He wrote his last novel there at the inn.
// It is hard to erase blue or red ink.
// Write at once or you may forget it.
// The doorknob was made of bright clean brass.
// They took the axe and the saw to the forest.
// Jazz and swing fans like fast music.
// The map had an X that meant nothing.
// Some ads serve to cheat buyers.
// On the islands the sea breeze is soft and mild.
// Add salt before you fry the egg.
// The houses are built of red clay bricks.
// Ducks fly north but lack a compass.
// He used the lathe to make brass objects.
// The vane on top of the pole revolved in the wind.
// Let it burn, it gives us warmth and comfort.
// A castle built from sand fails to endure.
// Tack the strip of carpet to the worn floor.
// Pour the stew from the pot into the plate.
// Each penny shone like new.
// The dirt piles were lines along the road.
// The logs fell and tumbled into the clear stream.
// Just hoist it up and take it away,
// Our plans right now are hazy.
// It takes a good trap to capture a bear.
// Feed the white mouse some flower seeds.
// The thaw came early and freed the stream.
// He took the lead and kept it the whole distance.
// The key you designed will fit the lock.
// This plank was made for walking on.
