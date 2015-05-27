extern crate serde;
extern crate serde_yaml as yaml;

mod structs;
mod ser_data;

#[test]
fn serialization() {
    let d = structs::Data1::default();

    println!("HAVE\n{}", yaml::to_string_pretty(&d).unwrap());
    println!("WANT\n{}", ser_data::DATA1_DEFAULT);

    assert_eq!(yaml::to_string_pretty(&d).unwrap(), ser_data::DATA1_DEFAULT);
}