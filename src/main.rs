#![allow(dead_code)]
use logos::Logos;
// use std::fs;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(r"\w+")]
    Word,

    #[regex(r"\W+")]
    Space,

    #[error]
    Error,
}

#[derive(Debug)]
struct Node {
    wood: Wood,
}

#[derive(Debug)]
enum Wood {
    Root(Root),
    Branch(Branch),
    Leaf(Leaf),
}

#[derive(Debug)]
struct Root {}

#[derive(Debug)]
enum Branch {
    Title(Title),
    Paragraph(Paragraph),
}

#[derive(Debug)]
enum Leaf {
    Text(Text),
}

#[derive(Debug)]
struct Title {}

#[derive(Debug)]
struct Paragraph {}

#[derive(Debug)]
struct Text {}

fn main() {
    println!("Starting");
    let input = "the quick brown fox";
    let root_wood = Wood::Root(Root {});
    let root_node = Node { wood: root_wood };
    j
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
