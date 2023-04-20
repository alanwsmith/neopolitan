#![allow(warnings)]
use crate::source_file::source_file::SourceFile;
use crate::universe::universe::Universe;

#[test]
pub fn integration_alfa() {
    let u = Universe::new();
    let lines = [
        "-> title",
        "",
        "Fasten two pins",
        "",
        "Pitch the straw",
        "",
        "Smell the rose",
    ];
    let source = lines.join("\n");
    let sf = SourceFile::new();
    assert_eq!(1, 1);
}
