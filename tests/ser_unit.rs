#![allow(non_camel_case_types)]
extern crate serde_yaml as yaml;
extern crate serde;

use std::collections::HashMap;

mod structs;

use yaml::ser::{PresentationDetails, DocumentIndicatorStyle, NullScalarStyle, FlowScalarStyle,
                ScalarStyle, StructureStyle, EscapeFormat};

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
    opts.scalar_value_details.style = ScalarStyle::Flow(0, 
                                    FlowScalarStyle::DoubleQuote(EscapeFormat::YAML));
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

#[test]
fn mapping_explicit_entry_mode() {
    panic!("TODO")    
}

#[test]
#[should_panic]
fn mapping_complex_keys_auto_explicit_entry() {
    // This would require us to build a repreesntation of the entire mapping key before we put 
    // down any character related to the key: value mapping. This requires us to have 
    // some sort of YAML::Value, which we simly don't have right now.
    panic!("We are currently unable to enforce using eplicit mapping entries for complex keys")
}

#[test]
fn json_yaml_auto_escape() {
    let json_opts = PresentationDetails::json();
    let yaml_opts = PresentationDetails::yaml();

    for &(source, want_json, want_yaml) in  [
                                             ("a",      r#""a""#,           "a"),
                                             ("",       r#""""#,            "''"),
                                             ("---",    r#""---""#,         "'---'"),
                                             ("...",    r#""...""#,         "'...'"),
                                             (" ",      r#"" ""#,           r#"' '"#),
                                             ("\\",     r#""\\""#,          r#""\\""#),
                                             ("\"",     r#""\"""#,          r#""\"""#),
                                             // just one of many special characters
                                             ("#",      r##""#""##,           "'#'"),
            // '''' would be OK for yaml, but requires a full pre-pass on the string 
            // which we don't do !
                                             ("'",      r#""'""#,           r#""'""#),
                                             ("\x00",   r#""\u0000""#,      r#""\0""#),
                                             ("\x01",   r#""\u0001""#,      r#""\x01""#),
                                             ("\x02",   r#""\u0002""#,      r#""\x02""#),
                                             ("\x03",   r#""\u0003""#,      r#""\x03""#),
                                             ("\x04",   r#""\u0004""#,      r#""\x04""#),
                                             ("\x05",   r#""\u0005""#,      r#""\x05""#),
                                             ("\x06",   r#""\u0006""#,      r#""\x06""#),
                                             ("\x07",   r#""\u0007""#,      r#""\a""#),
                                             ("\x08",   r#""\b""#,          r#""\b""#),
                                             ("\x09",   r#""\t""#,          r#""\t""#),
                                             ("\x0a",   r#""\n""#,          r#""\n""#),
                                             ("\x0b",   r#""\u000b""#,      r#""\v""#),
                                             ("\x0c",   r#""\f""#,          r#""\f""#),
                                             ("\x0d",   r#""\r""#,          r#""\r""#),
                                             ("\x0e",   r#""\u000e""#,      r#""\x0E""#),
                                             ("\x0f",   r#""\u000f""#,      r#""\x0F""#),

                                             ("\x1b",   r#""\u001b""#,      r#""\e""#),
                                             ("\u{a0}", r#""\u00a0""#,      r#""\_""#),
                                             ("\u{85}", r#""\u0085""#,      r#""\N""#),
                                             ("\u{2028}", r#""\u2028""#,      r#""\L""#),
                                             ("\u{2029}", r#""\u2029""#,      r#""\P""#),

                                             // whitespace
                                             ("  foo", r#""  foo""#,      r#"'  foo'"#),
                                             ("foo  ", r#""foo  ""#,      r#"'foo  '"#),
                                             ("foo  bar", r#""foo  bar""#,      r#"foo  bar"#),
                                             ("foo'bar", r#""foo'bar""#,      r#""foo'bar""#),
                                             ("\nfoo", r#""\nfoo""#,      r#""\nfoo""#),
             // non-printables: we do not escape them as most editors show them just fine !
             // Interestingly chinese characters seem 'unprintable' when lookging
             // at the libyaml implementation. They deserialize fine though ... .
                                             ("😀", "\"\u{01F600}\"",    "\u{01F600}"),
                                             ("好",  "\"\u{597D}\"",      "\u{597D}"),
                                             ("س",   "\"\u{0633}\"",      "\u{0633}"),
            // Control characters are escaped though
                                             ("\u{9e}",   r#""\u009e""#,      r#""\x9E""#),
                                             ].iter() {
        assert_eq!(yaml::to_string_with_options(&source, &json_opts).unwrap(), want_json);
        assert_eq!(yaml::to_string_with_options(&source, &yaml_opts).unwrap(), want_yaml);
    }
}