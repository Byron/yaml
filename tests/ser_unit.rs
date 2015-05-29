#![allow(non_camel_case_types)]
extern crate serde_yaml as yaml;

use yaml::ser::PresentationDetails;

#[test]
fn document_indicator_start() {
    let mut opts = yaml::ser::PresentationDetails::yaml();
    opts.document_indicator_style = Some(yaml::ser::DocumentIndicatorStyle::Start(None));
    let v: Option<u32> = None;

    let res = yaml::to_string_with_options(&v, opts).unwrap();
    println!("HAVE:\n{:?}", res);

    assert_eq!(res, "--- null");
}