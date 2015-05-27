extern crate serde;
extern crate serde_yaml as yaml;

mod structs;

const DATA1_DEFAULT: &'static str = 
r#"i32: 0,
i64: 0,
u32: 0,
u64: 0,
f32: 0.0,
f64: 0.0,
string: "",
i32a: [],
hash: {}"#;


#[test]
fn serialization() {
    let d = structs::Data1::default();

    assert_eq!(yaml::to_string_pretty(&d).unwrap(), DATA1_DEFAULT);
}