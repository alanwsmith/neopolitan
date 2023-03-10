#![allow(warnings)]
use logos::Logos;
// use std::fs;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(r"-> P *\n *\n")]
    Paragraph,

    #[regex(r"\w+")]
    Word,

    #[regex(r"\W+")]
    Space,

    #[error]
    Error,
}

#[derive(Debug)]
enum Node {
    Page { children: Vec<Node> },
    Paragraph { children: Vec<Node> },
    TextSpan { text: String },
}

// #[derive(Debug)]
// enum Node {
//     Paragraph(Paragraph),
//     Page(Page),
//     TextSpan(TextSpan),
// }

// #[derive(Debug)]
// enum Wood {
//     Branch(Branch),
//     Leaf(Leaf),
// }

// #[derive(Debug)]
// enum Branch {
//     Title(Title),
//     Paragraph(Paragraph),
// }

// #[derive(Debug)]
// enum Leaf {
//     Text(Text),
// }

// #[derive(Debug)]
// struct Title {}

// #[derive(Debug)]
// struct Paragraph {
//     children: Vec<Node>,
// }

// #[derive(Debug)]
// struct Word {}

// #[derive(Debug)]
// struct Space {}

// #[derive(Debug)]
// struct Page {
//     children: Vec<Node>,
// }

#[derive(Debug)]
struct TextSpan {
    text: String,
}

fn main() {
    println!("Starting");
    let input = "-> P\n\nthe quick brown fox";
    let mut page = Node::Page { children: vec![] };
    let mut lex = Token::lexer(&input);
    parse_next(&mut lex, &mut page);
    dbg!(page);
    println!("Done.");
}

fn parse_next(
    mut lex: &mut dyn Iterator<Item = Token>,
    node: &mut Node,
) {
    match lex.next() {
        Some(Token::Paragraph) => {
            println!("paragraph");
            let mut text_span = Node::TextSpan {
                text: "".to_string(),
            };
            parse_next(&mut lex, &mut text_span);
            let mut paragraph = Node::Paragraph {
                children: vec![text_span],
            };

            // node.Paragraph.children.push(paragraph);

            // match lex.peekable().peek() {
            //     Some(Token::Word) => {
            //         let mut text_span =
            //             Node::TextSpan(TextSpan {
            //                 text: "".to_string(),
            //             });
            //         parse_next(&mut lex, &mut text_span);
            //     }
            //     Some(Token::Space) => {
            //         let mut text_span =
            //             Node::TextSpan(TextSpan {
            //                 text: "".to_string(),
            //             });
            //         parse_next(&mut lex, &mut text_span);
            //     }
            //     Some(_) => {}
            //     None => {}
            // }
            // parse_next(&mut lex, &mut paragraph);
        }

        Some(Token::Word) => {
            println!("word");
        }
        Some(Token::Space) => {
            println!("space");
        }
        Some(Token::Error) => {
            println!("error");
        }
        None => {
            println!("Found last thing");
        }
    }
}

// fn parseit(
//     mut lex: &mut dyn Iterator<Item = Token>,
//     node: &mut Node,
// ) {
//     match lex.next() {
//         Some(Token::TitleTag) => {
//             let mut new_text_node = Node {
//                 kind: Kind::Text(Text { value: vec![] }),
//             };
//             let mut new_node = Node {
//                 kind: Kind::Title(Title {
//                     children: vec![new_text_node],
//                 }),
//             };
//             parseit(&mut lex, &mut new_node);
//             new_node.kind.children.push(new_node);
//         }
//         Some(Token::Word(_)) => {
//             println!("Word");
//         }
//         Some(Token::Error) => println!("Error"),
//         None => println!("None"),
//     }
//     // if let Some(x) = lex.next() {
//     //     dbg!(x.slice())
//     // }
//     // TitleTag => println!("asdf"),
//     // Error => println!("werwerwer")
//     // }
//     println!("HEREREERR");
// }

// fn main() {
//     println!("Starting");
//     let input_alfa = fs::read_to_string(
//         "_test_files/scratchpad/alfa.neo",
//     )
//     .unwrap();
//     let mut page = Node {
//         kind: Kind::Page(Page { children: vec![] }),
//     };
//     let mut lex = Token::lexer(&input_alfa);
//     parseit(&mut lex, &mut page);
//     dbg!(page);
//     println!("Done.");
// }

// #[derive(Debug)]
// struct Title {
//     children: Vec<Node>,
// }

// #[derive(Debug)]
// struct Paragraph {}

// #[derive(Debug)]
// struct Text {
//     value: Vec<String>,
// }

// #[derive(Logos, Debug, PartialEq)]
// enum Token<'a> {
//     #[regex(r"(?i)-> TITLE *\n *\n")]
//     TitleTag,
//     // #[regex(r"(?i)-> T[^I]")]
//     // TextTag,
//     // #[regex(r" ")]
//     // Space,
//     // #[regex(r"\n")]
//     // Newline,
//     #[regex(r"\w+")]
//     Word(&'a str),
//     #[error]
//     Error,
// }

// #[derive(Debug)]
// enum Kind {
//     Page(Page),
//     Title(Title),
//     Paragraph(Paragraph),
//     Text(Text),
// }

// #[derive(Debug)]
// struct Page {
//     children: Vec<Node>,
// }

// #[derive(Debug)]
// enum Content {
//     Title(Title),
//     Paragraph(Paragraph),
// }

// fn parseit(
//     mut lex: &mut dyn Iterator<Item = Token>,
//     node: &mut Node,
// ) {
//     match lex.next() {
//         Some(Token::TitleTag) => {
//             let mut new_text_node = Node {
//                 kind: Kind::Text(Text { value: vec![] }),
//             };
//             let mut new_node = Node {
//                 kind: Kind::Title(Title {
//                     children: vec![new_text_node],
//                 }),
//             };
//             parseit(&mut lex, &mut new_node);
//             new_node.kind.children.push(new_node);
//         }
//         Some(Token::Word(_)) => {
//             println!("Word");
//         }
//         Some(Token::Error) => println!("Error"),
//         None => println!("None"),
//     }
//     // if let Some(x) = lex.next() {
//     //     dbg!(x.slice())
//     // }
//     // TitleTag => println!("asdf"),
//     // Error => println!("werwerwer")
//     // }
//     println!("HEREREERR");
// }

// while let Some(x) = lex.next() {
//     dbg!(x);
//     dbg!(lex.peek());
// }

// dbg!(lex.peek().unwrap());
// dbg!(lex.next());
// dbg!(lex.peek());

// dbg!(tokens);
// let current_index: usize = 0;
// process_token(tokens, current_index);

// let mut p = Page { content: vec![] };

// for token in tokens.iter() {
//     // p.content.push(token)
//     // dbg!(&token);
// }
// dbg!(p);
//

// fn process_token<T: std::fmt::Debug>(
//     tokens: Vec<T>, current_index: usize,
// ) {
//     if current_index < tokens.len() {
//         let token = tokens.get(current_index);
//         let thing = token.unwrap();
//         dbg!(thing);
//         match thing {
//             Token::TitleTag => println!("HERERER"),
//             _ => println!("HEREEQWEQWE"),
//         }
//         let next_index = current_index + 1;
//         process_token(tokens, next_index);
//     }
//     // }
//     // else {
//     //     println!("done.");
//     // }
// }
