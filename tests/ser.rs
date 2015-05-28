#![allow(non_camel_case_types)]
extern crate serde;
extern crate serde_yaml as yaml;

mod structs;
mod ser_data;

#[test]
fn ser_yaml() {
    let d = structs::Data1::default();

    println!("HAVE\n{}", yaml::to_string(&d).unwrap());
    println!("WANT\n{}", ser_data::DATA1_DEFAULT);

    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::DATA1_DEFAULT);
}

#[test]
fn ser_json() {
    let d = structs::Data1::default();
    // TODO(stt) need differnt options
    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::DATA1_DEFAULT_JSON);
}

#[test]
fn ser_tuple() {
    let d = ("string", 5u32, 3.2f32);

    // TODO(stt) need differnt options
    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::LIST1_DEFAULT);
}

#[test]
fn example_2_1() {
    let d = &["Mark McGwire", 
              "Sammy Sosa", 
              "Ken Griffey"];

    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::EXAMPLE_2_1);
}

#[test]
fn example_2_2() {
    let d = structs::Example_2_2::default();

    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::EXAMPLE_2_2);
}

#[test]
fn example_2_3() {
    let d = structs::Example_2_3::default();

    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::EXAMPLE_2_3);
}

#[test]
fn example_2_4() {
    let d = structs::Example_2_4::default();

    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::EXAMPLE_2_4);
}

#[test]
fn example_2_5() {
    let d = (
        ("name"        , "hr", "avg"),
        ("Mark McGwire", 65, 0.278),
        ("Sammy Sosa"  , 63, 0.288),
    );
    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::EXAMPLE_2_5);
}

#[test]
fn example_2_6() {
    let d = structs::Example_2_6::default();

    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::EXAMPLE_2_6);
}

#[test]
fn example_2_7() {
    let _ = structs::Example_2_7::default();

    panic!("Multi-document handling needs implementation");
}

#[test]
fn example_2_8() {
    let _ = structs::Example_2_8::default();

    panic!("Multi-document handling needs implementation");
}

#[test]
fn example_2_9() {
    let d = structs::Example_2_9::default();

    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::EXAMPLE_2_9);
}

#[test]
#[should_panic]
fn example_2_10() {
    panic!("Anchors are not supported (neither they seem supported in PyYaml for ser): {}", 
            ser_data::EXAMPLE_2_10)
}

#[test]
fn example_2_11() {
    let d = structs::Example_2_11::default();

    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::EXAMPLE_2_11);
}

#[test]
fn example_2_12() {
    let d = structs::example_2_12_new();

    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::EXAMPLE_2_12);
}

#[test]
fn example_2_13() {
    let d = r#"\//||\/||\n// ||  ||__"#;

    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::EXAMPLE_2_13);
}

#[test]
fn example_2_14() {
    let d = "Mark McGwire's year was crippled by a knee injury.";

    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::EXAMPLE_2_14);
}


#[test]
#[should_panic]
fn example_2_15() {
    panic!("We cannot serialize folded scalars with two different blank line styles: {}", 
            ser_data::EXAMPLE_2_15)
}

#[test]
#[should_panic]
fn example_2_16() {
    panic!("We cannot serialize scalars with different styles: {}", 
            ser_data::EXAMPLE_2_16)
}

#[test]
#[should_panic]
fn example_2_17() {
    panic!("We cannot serialize flow-style scalars with different styles: {}", 
            ser_data::EXAMPLE_2_17)
}


#[test]
#[should_panic]
fn example_2_18() {
    panic!("We cannot serialize multi-line flow-style scalars with different styles: {}", 
            ser_data::EXAMPLE_2_18)
}

#[test]
#[should_panic]
fn example_2_19() {
    panic!("We cannot serialize integers with multiple styles (using tags): {}", 
            ser_data::EXAMPLE_2_19)
}

#[test]
#[should_panic]
fn example_2_20() {
    panic!("We cannot serialize floats with multiple styles (using tags): {}", 
            ser_data::EXAMPLE_2_20)
}

#[test]
fn example_2_21() {
    panic!("Schema needed to specify how different kinds of values are serialized: {}", 
            ser_data::EXAMPLE_2_21)
}

#[test]
fn example_2_22() {
    panic!("Have no idea how dates should be handled (does schema help ?): {}", 
            ser_data::EXAMPLE_2_22)
}

#[test]
fn example_2_23() {
    panic!("Have no idea how explict tags work during serialization ?: {}", 
            ser_data::EXAMPLE_2_23)
}

#[test]
fn example_2_24() {
    panic!("How to handle global tags during serialization ?: {}", 
            ser_data::EXAMPLE_2_24)
}

#[test]
fn example_2_25() {
    panic!("We can't handle unorded sets as to us they look like a sequence: {}", 
            ser_data::EXAMPLE_2_25)
}

#[test]
fn example_2_26() {
    panic!("There are no ordered mappings in Rust, and if so, we wouldn't know: {}", 
            ser_data::EXAMPLE_2_26)
}

#[test]
#[should_panic]
fn example_2_27() {
    panic!("Even though this might work, we can't support anchors: {}", 
            ser_data::EXAMPLE_2_27)
}

#[test]
fn example_2_28() {
    let d = structs::example_2_28_new();

    assert_eq!(yaml::to_string(&d).unwrap(), ser_data::EXAMPLE_2_28);
}