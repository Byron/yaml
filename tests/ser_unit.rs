#![allow(non_camel_case_types)]
extern crate serde_yaml as yaml;
extern crate serde;

use std::collections::HashMap;

mod structs;

use yaml::ser::{PresentationDetails, DocumentIndicatorStyle, NullScalarStyle, FlowScalarStyle,
                ScalarStyle, StructureStyle};

#[test]
fn document_indicator_start_and_null() {
    let mut opts = PresentationDetails::yaml();
    opts.document_indicator_style = Some(DocumentIndicatorStyle::Start(None));
    let v: Option<u32> = None;

    assert_eq!(opts.mapping_details.null_style, NullScalarStyle::HideValue);
    opts.mapping_details.null_style = NullScalarStyle::Show;
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "--- null");

    opts.scalar_value_details.style = ScalarStyle::Flow(0, FlowScalarStyle::Plain);
    opts.scalar_value_details.explicit_tag = true;
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "--- !!null null");
    opts.scalar_value_details.explicit_tag = false;

    opts.scalar_value_details.style = ScalarStyle::Flow(0, FlowScalarStyle::SingleQuote);
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "--- !!null 'null'");
    opts.scalar_value_details.style = ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote);
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "--- !!null \"null\"");

    opts.mapping_details.null_style = NullScalarStyle::HideValue;
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "---");
    opts.mapping_details.null_style = NullScalarStyle::HideEntry;
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "---");


    opts.document_indicator_style = Some(DocumentIndicatorStyle::StartEnd(None));
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "---\n...");
}


#[test]
fn empty_sequence_flow_block() {

    // empty sequence, with or without document start
    for style in &[StructureStyle::Flow, StructureStyle::Block] {
        let mut opts = PresentationDetails::yaml();
        opts.sequence_details.style = style.clone();

        let v: &[u8] = &[];
        assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "[]");

        opts.document_indicator_style = Some(DocumentIndicatorStyle::Start(None));
        assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "--- []");
    }
}


#[test]
fn empty_mapping_flow_block() {
    // empty sequence, with or without document start
    for style in &[StructureStyle::Flow, StructureStyle::Block] {
        let mut opts = PresentationDetails::yaml();
        opts.sequence_details.style = style.clone();

        let v = HashMap::<String, String>::new();
        assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "{}");

        opts.document_indicator_style = Some(DocumentIndicatorStyle::Start(None));
        assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "--- {}");
    }
}

#[test]
fn sequence_block() {
    let mut opts = PresentationDetails::yaml();
    let v = &[Option::None::<u32>, None];

    // hidden null (default)
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "-\n-");

    opts.mapping_details.null_style = NullScalarStyle::Show;
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "- null\n- null");

    opts.document_indicator_style = Some(DocumentIndicatorStyle::Start(None));
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "---\n- null\n- null");

    opts.mapping_details.null_style = NullScalarStyle::HideValue;
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "---\n-\n-");

    opts.document_indicator_style = Some(DocumentIndicatorStyle::StartEnd(None));
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "---\n-\n-\n...");
}

#[test]
fn mapping_block() {
    use structs::DualOptKey;
    let mut opts = PresentationDetails::yaml();
    let v = DualOptKey { key1: None, key2: None };

    // null is hidden by default
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "key1:\nkey2:");


    opts.document_indicator_style = Some(DocumentIndicatorStyle::Start(None));
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "---\nkey1:\nkey2:");


    opts.document_indicator_style = Some(DocumentIndicatorStyle::StartEnd(None));
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "---\nkey1:\nkey2:\n...");


    opts.mapping_details.null_style = NullScalarStyle::Show;
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), 
               "---\nkey1: null\nkey2: null\n...");

    // hiding all null values means we end up with an empty dict, which needs 
    // to be presented in flow mode
    opts.mapping_details.null_style = NullScalarStyle::HideEntry;
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(),
               "---\n{}\n...");

    let v = DualOptKey { key1: None, key2: Some(42) };
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(),
               "---\nkey2: 42\n...");
}


#[test]
fn sequence_flow() {
    use structs::SingleOptKey;

    let mut opts = PresentationDetails::yaml();
    opts.sequence_details.style = StructureStyle::Flow;

    let v = &[Option::None::<SingleOptKey>, None];

    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "[ null, null ]");

    // flow mode enforce flow mode for all nested structures
    // hiding of null values is done if possible, no matter what
    let v = &[Option::None::<structs::SingleOptKey>, Some(SingleOptKey { key: None })];
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "[ null, { key: } ]");
}

#[test]
fn mapping_flow() {
    use structs::DualOptKey;
    let mut opts = PresentationDetails::yaml();
    opts.mapping_details.details.style = StructureStyle::Flow;
    let v = DualOptKey { key1: None, key2: None };

    // null is hidden by default
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "{ key1:, key2: }");


    opts.document_indicator_style = Some(DocumentIndicatorStyle::Start(None));
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "--- { key1:, key2: }");


    opts.document_indicator_style = Some(DocumentIndicatorStyle::StartEnd(None));
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), "--- { key1:, key2: }\n...");


    opts.mapping_details.null_style = NullScalarStyle::Show;
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(), 
               "--- { key1: null, key2: null }\n...");

    // hiding all null values means we end up with an empty dict, which needs 
    // to be presented in flow mode
    opts.mapping_details.null_style = NullScalarStyle::HideEntry;
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(),
               "--- {}\n...");

    let v = DualOptKey { key1: None, key2: Some(42) };
    assert_eq!(yaml::to_string_with_options(&v, &opts).unwrap(),
               "--- { key2: 42 }\n...");
}