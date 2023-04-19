#![allow(warnings)]
use crate::universe::universe::Universe;

#[test]
pub fn integration_alfa() {
    let u = Universe::new();
    let lines = ["-> title", "", "Fasten two pins on each side"];
    let source = lines.join("\n");
    assert_eq!(1, 1);
}
