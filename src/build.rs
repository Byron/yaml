extern crate syntex;
extern crate serde_codegen;

use std::path::Path;

fn main() {
    let src = Path::new("tests/structs/in.rs");
    let dst = Path::new("tests/structs/out.rs");

    let mut registry = syntex::Registry::new();

    serde_codegen::register(&mut registry);
    registry.expand("serde_yaml", &src, &dst).unwrap();
}