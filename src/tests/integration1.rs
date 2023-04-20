#![allow(unused_imports)]
use crate::block::block::*;
use crate::parse::parse::parse;
use crate::section::section::*;
use crate::source_file::source_file::SourceFile;
use crate::universe::create_env::*;
use crate::universe::universe::Universe;
use minijinja::context;
use std::fs;

#[test]
pub fn integration1() {
    // There's no test here, this just outputs
    // some HTML to look at as a smoke test
    let mut u = Universe::new();
    u.env = Some(create_env("./src/tests/templates"));
    let source = fs::read_to_string("./src/tests/integration1.neo").unwrap();
    let mut sf = SourceFile::new();
    sf.raw_data = Some(source);
    sf.parsed = parse(sf.raw_data.as_ref().unwrap().as_str()).unwrap().1;
    let tmpl = u
        .env
        .as_ref()
        .unwrap()
        .get_template("wrappers/default.j2")
        .unwrap();
    let content = &sf.output(&u).unwrap();
    let output = tmpl.render(context!(content)).unwrap().to_string();
    fs::write("sites/test/index.html", output).unwrap();
}
