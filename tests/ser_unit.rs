#![allow(non_camel_case_types)]
extern crate serde_yaml as yaml;

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
fn empty_sequence() {

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