use std::io;
use std::string::FromUtf8Error;
use std::borrow::Borrow;
use std::fmt;
use num;

use serde::ser;

static YAML_ESCAPE_FORMAT: EscapeFormat = EscapeFormat::YAML;
// NOTE: The escape format doesn't actually matter to us !
static FLOW_DOUBLE_QUOTE: FlowScalarStyle = FlowScalarStyle::DoubleQuote(EscapeFormat::YAML);
static FLOW_SINGLE_QUOTE: FlowScalarStyle = FlowScalarStyle::SingleQuote;

const DOCUMENT_START: &'static str = "---";
const DOCUMENT_END: &'static str = "...";

/// The only allowed indentation character
const INDENT: &'static [u8] = b" ";

/// Identifies the type of structure we are currently in information 
#[derive(Clone)]
enum StructureKind {
    /// We are looking at a sequence
    Sequence,
    /// We are within a Mapping
    Mapping,
}

/// Primary state of the serializer to help assure proper call order
#[derive(Debug)]
enum State {
    Blank,
    InStream,
    InDocument,
    OutDocument,
}

/// A receiver for serialization events which are translated into a stream of characters, 
/// destined to serialize value types.
pub struct Serializer<W, D = Borrow<PresentationDetails>> 
    where D: Borrow<PresentationDetails>
{
    writer: W,
    // TODO(ST): this information is equivalent to stack.len() ... .
    current_indent: usize,
    opts: D,
    buf: String,
    open_structs_in_flow_style: usize,
    stack: Vec<StructureKind>,

    state: State,

    /// used to signify if we should print a comma when we are walking through a
    /// sequence.
    first_structure_elt: bool,

    /// Signals that we still have to serialize something - there was no call to anything
    /// that would fill the document yet. It's used to help putting a first linebreak only 
    /// if needed
    is_empty_line: bool,

    /// true while we are serializing a mapping key
    is_mapping_key: bool,

    /// While true, we signal that all serialized values are going to be part of a mapping key
    /// If the key is not simple enough, the mapping style may be forced to be either an 
    /// explicit mapping entry (if key is complex), or to use non-plain flow mode as escapes might
    /// be needed.
    is_complex_key: bool,
}

impl<W> Serializer<W, PresentationDetails>
    where W: io::Write,
{
    /// Creates a new YAML serializer.

    pub fn new(writer: W) -> Self {
        Serializer::with_options(writer, PresentationDetails::default())
    }
}


impl<W, D> Serializer<W, D>
    where W: io::Write,
          D: Borrow<PresentationDetails>
{
    /// Creates a new YAML visitor whose output will be written to the writer
    /// specified.
    pub fn with_options(writer: W, options: D) -> Self {
        Serializer {
            writer: writer,
            current_indent: 0,
            opts: options,
            buf: String::with_capacity(128),
            first_structure_elt: false,
            is_empty_line: true,
            is_mapping_key: false,
            is_complex_key: false,
            stack: Vec::with_capacity(10),
            open_structs_in_flow_style: 0,
            state: State::Blank,
        }
    }

    /// Unwrap the `Writer` from the `Serializer`.
    pub fn into_inner(self) -> W {
        self.writer
    }

    /// Returns either a Flow style if we are currently in enforced flow mode, or 
    /// the given structure style
    fn flow_style_or(&self, this_style: &StructureStyle) -> StructureStyle {
        if self.open_structs_in_flow_style == 0 {
            this_style.clone()
        } else {
            StructureStyle::Flow
        }
    }

    fn open(&mut self, kind: StructureKind) -> io::Result<()>
    {
        if self.is_mapping_key {
            self.is_complex_key = true;
        }

        self.stack.push(kind.clone());

        // REVIEW(ST): consider merging these branches - they look similar
        // have to wait until internal data structures stabilize
        self.current_indent += 1;
        self.first_structure_elt = true;
        let mut line_remains_empty = false;
        let open_ascii: &str = 
            match kind {
                StructureKind::Sequence => {
                    match self.flow_style_or(&self.opts.borrow().sequence_details.style) {
                        // In block mode, we always start on a new line
                        StructureStyle::Block => {
                            line_remains_empty = true;
                            if self.is_empty_line { 
                                ""
                            } else {
                                self.opts.borrow().format.line_break.as_ref()
                            }
                        },
                        StructureStyle::Flow => {
                            self.open_structs_in_flow_style += 1;
                            if self.is_empty_line {
                                "["
                            } else {
                                " ["
                            }
                        }
                    }
                },
                StructureKind::Mapping => {
                    match self.flow_style_or(&self.opts.borrow().mapping_details.details.style) {
                        // In block mode, we always start on a new line
                        StructureStyle::Block => {
                            line_remains_empty = true;
                            if self.is_empty_line { 
                                ""
                            } else {
                                self.opts.borrow().format.line_break.as_ref()
                            }
                        },
                        StructureStyle::Flow => {
                            self.open_structs_in_flow_style += 1;
                            if self.is_empty_line {
                                "{"
                            } else {
                                " {"
                            }
                        }
                    }
                }
            };

        self.is_empty_line = line_remains_empty;
        encode_str(&mut self.writer, &self.opts.borrow().format.encoding, open_ascii)
    }

    fn emit_newline(&mut self) -> io::Result<()> {
        self.is_empty_line = true;
        encode_str(&mut self.writer, &self.opts.borrow().format.encoding, 
                     &self.opts.borrow().format.line_break)
    }

    /// Place a separator suitable for sequences or mappings, based on the current structure
    /// and presentation options
    ///
    /// **REVIEW(ST)**: currently it looks like we just need the style, not so much 
    /// the StructureKind
    fn elt_sep(&mut self, first: bool, kind: StructureKind) -> io::Result<()>
    {
        if first {
            return Ok(())
        }

        let style = self.flow_style_or(
            match kind {
                StructureKind::Sequence => &self.opts.borrow().sequence_details.style,
                StructureKind::Mapping => &self.opts.borrow().mapping_details.details.style,
            }
        );

        match style {
            StructureStyle::Block => {
                self.emit_newline()
                // TODO(ST): Actual indentation handling ... this is still from JSON
                // indent(&mut self.writer, self.current_indent * 
                //        self.opts.borrow().format.spaces_per_indentation_level)
            },
            StructureStyle::Flow => {
                self.is_empty_line = false;
                encode_str(&mut self.writer, &self.opts.borrow().format.encoding, ",")
            }
        }
    }

    fn colon(&mut self) -> io::Result<()>
    {
        self.is_empty_line = false;
        encode_str(&mut self.writer, &self.opts.borrow().format.encoding, ":")
    }

    fn close(&mut self, kind: StructureKind) -> io::Result<()>
    {
        if self.is_mapping_key {
            self.is_complex_key = false;
        }

        self.current_indent -= 1;
        self.stack.pop().expect("pop() must match push()");

        // TODO(ST): Actual indentation handling
        // try!(indent(&mut self.writer, self.current_indent * 
        //                  self.opts.borrow().format.spaces_per_indentation_level));

        let close_ascii: &str = 
            match kind {
                StructureKind::Sequence => {
                    match self.flow_style_or(&self.opts.borrow().sequence_details.style) {
                        StructureStyle::Block => "",
                        StructureStyle::Flow => {
                            self.open_structs_in_flow_style -= 1;
                            " ]"
                        },
                    }
                },
                StructureKind::Mapping => {
                    match self.flow_style_or(&self.opts.borrow().mapping_details.details.style) {
                        StructureStyle::Block => "",
                        StructureStyle::Flow => {
                            self.open_structs_in_flow_style -= 1;
                            " }"
                        },
                    }
                }
            };

        encode_str(&mut self.writer, &self.opts.borrow().format.encoding, close_ascii)
    }

    /// Must be called once when starting to serialize any amount of documents.
    /// 
    /// **NOTE:** Failing to do so will cause a runtime panic.
    pub fn begin_stream(&mut self) -> io::Result<()> {
        match self.state {
            State::Blank => self.state = State::InStream,
            _ => panic!("Must be in blank state, found {:?}", self.state)
        }
        
        self.writer.write_all(self.opts.borrow().format.encoding.as_ref())
    }

    /// Must be called once before serializing any value.
    ///
    /// **NOTE:** Failing to do so will cause a runtime panic.
    pub fn begin_document(&mut self) -> io::Result<()> {
        match self.state {
            State::InStream => self.state = State::InDocument,
            State::OutDocument => {
                try!(self.emit_newline());
                self.state = State::InDocument;
            },
            _ => panic!("Must be in stream state, found {:?}", self.state)
        }

        match self.opts.borrow().document_indicator_style.clone() {
             Some(DocumentIndicatorStyle::Start(ref yaml_directive))
            |Some(DocumentIndicatorStyle::StartEnd(ref yaml_directive)) => {
                // We don't care about the actual type of it, as we support single-document only
                // TODO(ST) review this once multi-document modes are possible
                try!(
                    match *yaml_directive {
                        Some(ref yaml_directive) => {
                            try!(encode_str(&mut self.writer, 
                                              &self.opts.borrow().format.encoding,
                                              yaml_directive));
                            // need line-break after version directive
                            self.emit_newline()
                        },
                        None => Ok(())
                    });

                // We may assume that if called, there is at least one value coming.
                // Therefore we can put a space here by default, as values will not 
                // take care of that
                self.is_empty_line = false;
                encode_str(&mut self.writer, &self.opts.borrow().format.encoding, 
                             DOCUMENT_START)
            },
            None => Ok(()),
        }
    }

    /// The sibling of `begin_document()`, which must be called exactly once.
    ///
    /// **NOTE:** Failing to do so will cause a runtime panic on the next call to 
    /// `begin_document()`
    pub fn end_document(&mut self) -> io::Result<()> {
        match self.state {
            State::InDocument => self.state = State::OutDocument,
            _ => panic!("Must be in stream state, found {:?}", self.state)
        }

        match self.opts.borrow().document_indicator_style {
            Some(DocumentIndicatorStyle::StartEnd(_)) => {
                    self.is_empty_line = true;
                    try!(self.emit_newline());
                    encode_str(&mut self.writer, &self.opts.borrow().format.encoding,
                                 DOCUMENT_END)
                },
            _ => Ok(())
        }
    }

    fn encode_scalar<B>(&mut self, tag: Tag, mut opts: ScalarDetails, chars: B) -> io::Result<()>
        where B: AsRef<str>
    {
        let encoding = &self.opts.borrow().format.encoding;
        // If we have non-strings which are supposed to be printed in non-plain mode, 
        // the tag is a requirement, no matter what. Otherwise these values would be interpreted
        // as strings (thus we need the tag to maintain the original type)
        let have_string = 
            if let Tag::Str = tag {
                true
            } else {
                if let ScalarStyle::Flow(_, ref flow_style) = opts.style {
                    opts.explicit_tag = 
                        match *flow_style {
                            FlowScalarStyle::Plain => opts.explicit_tag,
                            _ => true
                        };
                }
                false
            };

        if opts.explicit_tag {
            if !self.is_empty_line {
                try!(encode_str(&mut self.writer, encoding, " "));
            }
            try!(encode_str(&mut self.writer, encoding, tag.as_ref()));
            self.is_empty_line = false;
        }

        // just to make setting it easier
        let is_empty_line = self.is_empty_line;
        self.is_empty_line = false;
        match opts.style {
            ScalarStyle::Block(_) => panic!("TODO"),
            ScalarStyle::Flow(ref width, ref flow_style) => {
                let (flow_style, str_slice) = 
                    if have_string {
                        if let Some(enforced_flow_style)
                                        = escape_str_and_fold(chars.as_ref(), &mut self.buf,
                                                              *width, flow_style) {
                            // have escaped characters, possibly folded
                            (enforced_flow_style, self.buf.as_ref())
                        } else if chars.as_ref().len() != self.buf.len() {
                            // have fold only - style unaffected
                            (flow_style, self.buf.as_ref())
                        } else {
                            (flow_style, chars.as_ref())
                        }
                    } else {
                        // Any other scalars neither need folding, nor do they need escaping
                        (flow_style, chars.as_ref())
                    };

                if !is_empty_line {
                    try!(encode_str(&mut self.writer, encoding, " "));
                }
                try!(encode_str(&mut self.writer, encoding, flow_style));
                try!(encode_str(&mut self.writer, encoding, str_slice));
                encode_str(&mut self.writer, &self.opts.borrow().format.encoding, flow_style)
            },
        }
    }

    fn visit_any<T>(&mut self, tag: Tag, value: T) -> io::Result<()> 
        where T: fmt::Display
    {
        if self.is_complex_key {
            panic!("TODO(ST): complex keys")
        } else {
            // TODO(ST): use pre-allocated buffer ! Like a formatter that writes into the same,
            //           have the slow, but simple implementation for now
            let svd = self.opts.borrow().scalar_value_details.clone();
            self.encode_scalar(tag, svd, format!("{}", value))
        }
    }

    fn visit_float<T>(&mut self, tag: Tag, value: T) -> io::Result<()> 
        where T: num::Float + fmt::Display
    {
        //! TODO(ST): inf and nan handling is part of the YAML specification, so it shouldn't 
        //! be null, but instead be the required value. There is a lot going on 
        //! regarding float parsing and serialization, and we might be fine with the
        //! standard fmt::Display if we are lucky
        // For reference
        // use std::num::FpCategory;
        // match value.classify() {
        //     FpCategory::Nan | FpCategory::Infinite => wr.write_all(b"null"),
        //     _ => {
        //         let s = format!("{:?}", value);
        //         try!(wr.write_all(s.as_bytes()));
        //         if !s.contains('.') {
        //             try!(wr.write_all(b".0"))
        //         }
        //         Ok(())
        //     }
        // }
        self.visit_any(tag, value)
    }
}

impl<W, D> ser::Serializer for Serializer<W, D>
    where W: io::Write,
          D: Borrow<PresentationDetails>
{
    type Error = io::Error;

    fn visit_bool(&mut self, value: bool) -> io::Result<()> {
        if self.is_complex_key {
            panic!("TODO(ST): complex keys")
        } else {
            let svd = self.opts.borrow().scalar_value_details.clone();
            let bool_str = if value { &"true"[..] } else { &"false"[..] };
            self.encode_scalar(Tag::Bool, svd, bool_str)
        }
    }

    fn visit_isize(&mut self, value: isize) -> io::Result<()> {
        self.visit_any(Tag::Int, value)
    }

    fn visit_i8(&mut self, value: i8) -> io::Result<()> {
        self.visit_any(Tag::Int, value)
    }

    fn visit_i16(&mut self, value: i16) -> io::Result<()> {
        self.visit_any(Tag::Int, value)
    }

    fn visit_i32(&mut self, value: i32) -> io::Result<()> {
        self.visit_any(Tag::Int, value)
    }

    fn visit_i64(&mut self, value: i64) -> io::Result<()> {
        self.visit_any(Tag::Int, value)
    }

    fn visit_usize(&mut self, value: usize) -> io::Result<()> {
        self.visit_any(Tag::Int, value)
    }

    fn visit_u8(&mut self, value: u8) -> io::Result<()> {
        self.visit_any(Tag::Int, value)
    }

    fn visit_u16(&mut self, value: u16) -> io::Result<()> {
        self.visit_any(Tag::Int, value)
    }

    fn visit_u32(&mut self, value: u32) -> io::Result<()> {
        self.visit_any(Tag::Int, value)
    }

    fn visit_u64(&mut self, value: u64) -> io::Result<()> {
        self.visit_any(Tag::Int, value)
    }

    fn visit_f32(&mut self, value: f32) -> io::Result<()> {
        self.visit_float(Tag::Float, value)
    }

    fn visit_f64(&mut self, value: f64) -> io::Result<()> {
        self.visit_float(Tag::Float, value)
    }

    fn visit_char(&mut self, value: char) -> io::Result<()> {
        // FIXME: this allocation is required in order to be compatible with stable
        // rust, which doesn't support encoding a `char` into a stack buffer.
        // We might be able to do it once we have some sort of formatter for use 
        // See `visit_any()` for details
        self.visit_str(value.to_string().as_ref())
    }

    /// This is only called if the string is on the value side, or if it is 
    /// part of a generic mapping, like a hash_map.
    /// TODO(ST): we should know if we are a key of a map, or part of a complex mapping key
    ///           as long as we don't know, we actually use the wrong settings 
    ///           (should be scalar_key_details)
    fn visit_str(&mut self, value: &str) -> io::Result<()> {
        if self.is_complex_key {
            panic!("TODO(ST): complex keys")
        } else {
            let str_opts = 
                if self.is_mapping_key {
                    // TODO(ST): lookahead during parsing is just 1024, therefore we need
                    // explicit mapping entries for keys which are longer !
                    // NOTE: we just count bytes, and thus might cut off at smaller lengths ...  
                    if value.len() > 1023 {
                        panic!("TODO")
                    }
                    self.opts.borrow().scalar_key_details.clone()
                } else {
                    if value.len() < self.opts.borrow().small_scalar_string_value_width_threshold {
                        self.opts.borrow().small_scalar_string_value_details.clone()
                    } else {
                        self.opts.borrow().big_scalar_string_value_details.clone()
                    }
                };
            self.encode_scalar(Tag::Str, str_opts, value)
        }
    }

    fn visit_none(&mut self) -> io::Result<()> {
        self.visit_unit()
    }

    fn visit_some<V>(&mut self, value: V) -> io::Result<()>
        where V: ser::Serialize
    {
        value.serialize(self)
    }

    fn visit_unit(&mut self) -> io::Result<()> {
        let mut null_style = self.opts.borrow().mapping_details.null_style.clone();
        if self.open_structs_in_flow_style > 0 {
            if let Some(&StructureKind::Sequence) = self.stack.last() {
                null_style = NullScalarStyle::Show;
            }
        }

        match null_style {
            NullScalarStyle::HideValue
            |NullScalarStyle::HideEntry 
                => Ok(()),
            NullScalarStyle::Show => {
                let svd = self.opts.borrow().scalar_value_details.clone();
                self.encode_scalar(Tag::Null, svd, "null")
            },
        }
    }

    fn visit_enum_unit(&mut self, _name: &str, variant: &str) -> io::Result<()> {
        panic!("TODO(ST): should be like a unit, I guess");
        try!(self.open(StructureKind::Mapping));
        // we know the variant is a simple string, so we treat it that way
        {
            let skd = self.opts.borrow().scalar_key_details.clone(); // borrowchk :(
            try!(self.encode_scalar(Tag::Str, skd, variant));
        }
        try!(self.colon());
        try!(self.writer.write_all(b"[]"));
        self.close(StructureKind::Mapping)
    }

    fn visit_seq<V>(&mut self, mut visitor: V) -> io::Result<()>
        where V: ser::SeqVisitor,
    {
        match visitor.len() {
            Some(len) if len == 0 => {
                encode_str(&mut self.writer, &self.opts.borrow().format.encoding, 
                             if self.is_empty_line { &"[]"[..] }
                             else { &" []"[..]})
            }
            _ => {
                try!(self.open(StructureKind::Sequence));

                while let Some(()) = try!(visitor.visit(self)) { }

                self.close(StructureKind::Sequence)
            }
        }

    }

    fn visit_enum_seq<V>(&mut self, _name: &str, variant: &str, visitor: V) -> io::Result<()>
        where V: ser::SeqVisitor,
    {
        panic!("TODO(ST): should be like a sequence, I guess");
        try!(self.open(StructureKind::Mapping));
        {
            let skd = self.opts.borrow().scalar_key_details.clone(); // borrowchk :(
            try!(self.encode_scalar(Tag::Str, skd, variant));
        }
        try!(self.colon());
        try!(self.visit_seq(visitor));
        self.close(StructureKind::Mapping)
    }

    fn visit_seq_elt<T>(&mut self, value: T) -> io::Result<()>
        where T: ser::Serialize,
    {
        {
            let first = self.first_structure_elt;
            try!(self.elt_sep(first, StructureKind::Sequence));
        }
        self.first_structure_elt = false;

        if let StructureStyle::Block = self.flow_style_or(&self.opts.borrow()
                                                                    .sequence_details.style) {
            try!(encode_str(&mut self.writer, &self.opts.borrow().format.encoding, "-"));
            self.is_empty_line = false;
        }

        value.serialize(self)
    }

    fn visit_map<V>(&mut self, mut visitor: V) -> io::Result<()>
        where V: ser::MapVisitor,
    {
        match visitor.len() {
            Some(len) if len == 0 => {
                encode_str(&mut self.writer, &self.opts.borrow().format.encoding, 
                             if self.is_empty_line { &"{}"[..] }
                             else { &" {}"[..]})
            }
            _ => {
                try!(self.open(StructureKind::Mapping));

                while let Some(()) = try!(visitor.visit(self)) { }

                self.close(StructureKind::Mapping)
            }
        }
    }

    fn visit_enum_map<V>(&mut self, _name: &str, variant: &str, visitor: V) -> io::Result<()>
        where V: ser::MapVisitor,
    {
        panic!("TODO(ST): should be like a map, I guess");
        try!(self.open(StructureKind::Mapping));
        {
            let skd = self.opts.borrow().scalar_key_details.clone(); // borrowchk :(
            try!(self.encode_scalar(Tag::Str, skd, variant));
        }
        try!(self.colon());
        try!(self.visit_map(visitor));

        self.close(StructureKind::Mapping)
    }

    fn visit_map_elt<K, V>(&mut self, key: K, value: V) -> io::Result<()>
        where K: ser::Serialize,
              V: ser::Serialize,
    {
        {
            let first = self.first_structure_elt; // workaround borrowchk
            try!(self.elt_sep(first, StructureKind::Mapping));
        }
        self.first_structure_elt = false;

        let explicit_entries = self.opts.borrow().mapping_details.explicit_entries;
        if explicit_entries {
            let encoding = &self.opts.borrow().format.encoding;
            if !self.is_empty_line {
                try!(encode_str(&mut self.writer, encoding, " "));
            }
            try!(encode_str(&mut self.writer, encoding, "?"));
            self.is_empty_line = false;
        }

        self.is_mapping_key = true;
        try!(key.serialize(self));
        self.is_mapping_key = false;
        if explicit_entries {
            if let StructureStyle::Block = self.flow_style_or(&self.opts.borrow()
                                                              .mapping_details.details.style) {
                try!(self.emit_newline());
            }
        }
        try!(self.colon());
        value.serialize(self)
    }

    fn format() -> &'static str {
        "yaml"
    }
}

/// Encode the specified struct into a YAML `[u8]` writer.
pub fn to_writer<W, T>(writer: &mut W, value: &T) -> io::Result<()>
    where W: io::Write,
          T: ser::Serialize,
{
    to_writer_with_options(writer, value, PresentationDetails::default())
}

/// Encode the specified struct into a json `[u8]` writer, with the given 
/// options to define how the character stream should look like.
pub fn to_writer_with_options<W, T, D>(writer: &mut W, value: &T, 
                                    options: D) -> io::Result<()>
    where W: io::Write,
          T: ser::Serialize,
          D: Borrow<PresentationDetails>
{
    let mut ser = Serializer::with_options(writer, options);
    try!(ser.begin_stream());
    try!(ser.begin_document());
    try!(value.serialize(&mut ser));
    try!(ser.end_document());
    Ok(())
}

/// Encode the specified struct into a YAML `[u8]` buffer.
pub fn to_vec<T>(value: &T) -> Vec<u8>
    where T: ser::Serialize,
{
    // We are writing to a Vec, which doesn't fail. So we can ignore
    // the error.
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value).unwrap();
    writer
}

/// Encode the specified struct into a YAML `[u8]` buffer with the given options
/// to define how the character stream should look like.
pub fn to_vec_with_options<T, D>(value: &T, options: D) -> Vec<u8>
    where T: ser::Serialize,
          D: Borrow<PresentationDetails>
{
    let mut writer = Vec::with_capacity(128);
    to_writer_with_options(&mut writer, value, options).unwrap();
    writer
}

/// Encode the specified struct into a YAML `String` buffer.
pub fn to_string<T>(value: &T) -> Result<String, FromUtf8Error>
    where T: ser::Serialize
{
    let vec = to_vec(value);
    String::from_utf8(vec)
}

/// Encode the specified struct into a YAML `String` buffer with the given 
/// options to define how the character stream should look like.
///
/// *NOTE*: the encoding must be utf8 as this is the internal format
/// of Rust strings. If it is not the case, you will get FromUtf8Error result.
pub fn to_string_with_options<T, D>(value: &T, 
                                    options: D) -> Result<String, FromUtf8Error>
    where T: ser::Serialize,
          D: Borrow<PresentationDetails>
{
    let vec = to_vec_with_options(value, options);
    String::from_utf8(vec)
}

fn indent<W>(wr: &mut W, n: usize) -> io::Result<()>
    where W: io::Write,
{
    for _ in 0 .. n {
        try!(wr.write_all(INDENT));
    }

    Ok(())
}

fn encode_str<W, B>(writer: &mut W, encoding: &Encoding, chars: B) -> io::Result<()> 
    where W: io::Write,
          B: AsRef<str>
{
    if chars.as_ref().len() == 0 {
        return Ok(())
    }

    match *encoding {
        // str.as_bytes() is guaranteed to be encoded in UTF-8
        Encoding::Utf8(_) => writer.write_all(chars.as_ref().as_bytes()),
    }
}

/// Returns Some(Stylel) if the destination buffer `dst` contains at least one escaped character
/// from `src` which requires the given style.
/// Please note that the string in dst may also be folded, and thus increase in length, even if 
/// None is returned.
fn escape_str_and_fold(src: &str, dst: &mut String, max_fold_width: usize, 
                       in_style: &FlowScalarStyle) -> Option<&'static FlowScalarStyle>
{
    use std::fmt::Write;

    dst.clear();

    // NOTE(ST): Used in hopefully rare cases, so heap-allocation won't matter
    // If it does, it would be easy to pass the buf as argument so the heap memory can be reused.
    let mut buf = String::new();

    let mut style = None;
    let mut start = 0;

    let (escape_format, is_double_quoted) = match *in_style {
        FlowScalarStyle::DoubleQuote(ref escape_format) => {
            (escape_format, true)
        },
        FlowScalarStyle::SingleQuote => (&YAML_ESCAPE_FORMAT, false),
        FlowScalarStyle::Plain => {
            // Figure out if the string can be represented with the plain flow-style
            // Note that we don't have to care if it's already double-quote, as these strings 
            // can represent everything
            style = Some(&FLOW_SINGLE_QUOTE);

           'outer: 
            loop {
                if src.len() == 0 {
                    break;
                }
                // Check document start/end
                if src.len() > 2 {
                    if src.as_bytes()[..3] == *DOCUMENT_START.as_bytes() ||
                       src.as_bytes()[..3] == *DOCUMENT_END.as_bytes() {
                        break;
                   }
                }

                // means we don't start with whitespace
                let mut last_ws_char = usize::max_value();
                for (i, chr) in src.chars().enumerate() {
                    if i == 0 {
                        // start with whitespace ...
                        if chr.is_whitespace() {
                            break 'outer;
                        }
                        // or with a reserved character
                        match chr {
                              '#'|','|'['|']'|'{'|'}'|'&'|'*'
                             |'!'|'|'|'>'|'"'|'%'|'@'|'`'
                             |'?'|':'|'-' => {
                                break 'outer;
                             },
                             _ => (),
                        }
                    } else if chr.is_whitespace() {
                        last_ws_char = i;
                    }
                }// char loop

                if last_ws_char == src.len() - 1 {
                    break;
                }

                // We have found no reason to elevate the style from plain mode
                style = None;
                break;
            }// end breakout loop

            (&YAML_ESCAPE_FORMAT, false)
        },
    };


    match *escape_format {
        EscapeFormat::YAML => {
            for (i, chr) in src.char_indices() {
                let escaped = 
                    match chr {
                        '"' => "\\\"",
                        '\'' => "'",
                        '\\' => "\\\\",
                        '\x00' => "\\0",
                        '\x01' => "\\x01",
                        '\x02' => "\\x02",
                        '\x03' => "\\x03",
                        '\x04' => "\\x04",
                        '\x05' => "\\x05",
                        '\x06' => "\\x06",
                        '\x07' => "\\a",
                        '\x08' => "\\b",
                        '\x09' => "\\t",
                        '\x0a' => "\\n",
                        '\x0b' => "\\v",
                        '\x0c' => "\\f",
                        '\x0d' => "\\r",
                        '\x0e' => "\\x0E",
                        '\x0f' => "\\x0F",
                        '\x10' => "\\u0010",
                        '\x11' => "\\u0011",
                        '\x12' => "\\u0012",
                        '\x13' => "\\u0013",
                        '\x14' => "\\u0014",
                        '\x15' => "\\u0015",
                        '\x16' => "\\u0016",
                        '\x17' => "\\u0017",
                        '\x18' => "\\u0018",
                        '\x19' => "\\u0019",
                        '\x1a' => "\\u001a",
                        '\x1b' => "\\e",
                        '\x1c' => "\\u001c",
                        '\x1d' => "\\u001d",
                        '\x1e' => "\\u001e",
                        '\x1f' => "\\u001f",
                        '\x7f' => "\\u007f",
                        '\u{a0}' => "\\_",
                        '\u{85}' => "\\N",
                        '\u{2028}' => "\\L",
                        '\u{2029}' => "\\P",
                        c if c.is_control() => {
                            // no need to handle the 16 bit and 32bit cases, as we don't know them
                            // There are plenty of unprintable characters, but figuring them all
                            // out is difficult. According to pyyaml, chinese symbols are 
                            // unprintable, even though they are not !
                            debug_assert!(c < '\u{100}');
                            buf.clear();
                            fmt::write(&mut buf, format_args!("\\x{:02X}", c as u8)).unwrap();
                            buf.as_ref()
                        }
                        _ => { continue; }
                    };

                if start < i {
                    dst.write_str(&src[start..i]).unwrap();
                }

                if !is_double_quoted {
                    style = Some(&FLOW_DOUBLE_QUOTE);
                }
                dst.write_str(escaped).unwrap();

                start = i + chr.len_utf8();
            }
        },// end YAML
        EscapeFormat::JSON => {
            for (i, chr) in src.char_indices() {
                let escaped = 
                    match chr {
                        '"' => "\\\"",
                        '\'' => "'",
                        '\\' => "\\\\",
                        '\x00' => "\\u0000",
                        '\x01' => "\\u0001",
                        '\x02' => "\\u0002",
                        '\x03' => "\\u0003",
                        '\x04' => "\\u0004",
                        '\x05' => "\\u0005",
                        '\x06' => "\\u0006",
                        '\x07' => "\\u0007",
                        '\x08' => "\\b",
                        '\x09' => "\\t",
                        '\x0a' => "\\n",
                        '\x0b' => "\\u000b",
                        '\x0c' => "\\f",
                        '\x0d' => "\\r",
                        '\x0e' => "\\u000e",
                        '\x0f' => "\\u000f",
                        '\x10' => "\\u0010",
                        '\x11' => "\\u0011",
                        '\x12' => "\\u0012",
                        '\x13' => "\\u0013",
                        '\x14' => "\\u0014",
                        '\x15' => "\\u0015",
                        '\x16' => "\\u0016",
                        '\x17' => "\\u0017",
                        '\x18' => "\\u0018",
                        '\x19' => "\\u0019",
                        '\x1a' => "\\u001a",
                        '\x1b' => "\\u001b",
                        '\x1c' => "\\u001c",
                        '\x1d' => "\\u001d",
                        '\x1e' => "\\u001e",
                        '\x1f' => "\\u001f",
                        '\x7f' => "\\u007f",
                        '\u{a0}' => "\\u00a0",
                        '\u{85}' => "\\u0085",
                        '\u{2028}' => "\\u2028",
                        '\u{2029}' => "\\u2029",
                        c if c.is_control() => {
                            // no need for a surrogate pair as this category is only 8bit
                            debug_assert!(c < '\u{10000}');
                            buf.clear();
                            fmt::write(&mut buf, format_args!("\\u{:04x}", c as u16)).unwrap();
                            buf.as_ref()
                        }
                        _ => { continue; }
                    };

                if start < i {
                    dst.write_str(&src[start..i]).unwrap();
                }

                if !is_double_quoted {
                    style = Some(&FLOW_DOUBLE_QUOTE);
                }
                dst.write_str(escaped).unwrap();

                start = i + chr.len_utf8();
            }
        }// END JSON
    }
    
    if start != src.len() {
        dst.write_str(&src[start..]).unwrap();
    }

    style
}




/// Defines the way we shall use to preserve newlines within folded scalar block 
/// literals.
///
/// [YAML Spec](http://www.yaml.org/spec/1.2/spec.html#id2760844)
#[derive(Debug, PartialEq, Clone)]
pub enum FoldedBlockScalarNewlinePreservationMode {
    /// A line break is indicated by a single blank line
    BlankLines,
    /// A line break is indicated by indenting a line by one additional level.
    ///
    /// **TODO(ST)**: is it depending on the `spaces_per_indentation_level` flag, or should this 
    /// just be hardcoded to (say) two spaces ?
    Indentation,
}


/// Scalar content can be written in block notation, using a literal style (indicated by `|`)
/// where all line breaks are significant. Alternatively, they can be written with the folded 
/// style (denoted by `>`) where each line break is folded to a space unless it ends an empty or 
/// a more-indented line.
/// This style only works within a Block structure
///
/// [YAML Spec](http://www.yaml.org/spec/1.2/spec.html#style/block/)
#[derive(Debug, PartialEq, Clone)]
pub enum BlockScalarStyle {
    /// Literal scalar blocks are indicated by the `|` character within the YAML file
    /// and cause line-breaks to remain significant.
    Literal,
    /// Folded scalar blocks are indicated by the `>` character within the YAML file
    /// and will translate line-breaks into spaces unless it ends with an empty or a more 
    /// indented line.
    ///
    /// The *first tuple struct member* is the maximum width hint of a scalar line within 
    /// multi-line strings. It is a hint only, and if too small, some
    /// scalar lines might end up with greater width than specified here.
    ///
    /// The *second tuple struct member* defines the way newlines are preserved within 
    /// the folded scalar.
    Folded(usize, FoldedBlockScalarNewlinePreservationMode),
}


/// Identifies how we will escape unprintable characters within strings
#[derive(Debug, PartialEq, Clone)]
pub enum EscapeFormat {
    /// Use YAML encoding rules. Based on these 
    /// [escaping rules](http://www.yaml.org/spec/1.2/spec.html#id2776092)
    /// and the YAML [character set](http://www.yaml.org/spec/1.2/spec.html#id2770814)
    ///
    /// Reference implementation is [libyaml](https://goo.gl/4MgVbw)
    YAML,
    /// Use JSON encoding rules, as (somewhat indirectly) specified on
    /// [json.org](http://json.org/).
    /// 
    /// Reference implementation is 
    /// [yaml-rust](https://goo.gl/RqhY1W)
    JSON
}

/// Defines how scalars are presented in Flow-Style.
/// All scalars can span multiple lines, which are folded automatically.
/// Therefore, the Flow style is a form of a folded scalar style.
///
/// [YAML Spec](http://www.yaml.org/spec/1.2/spec.html#id2786942)
#[derive(Debug, PartialEq, Clone)]
pub enum FlowScalarStyle {
    /// Scalars are not enclosed by identifiers at all, e.g. `key: value`
    Plain,
    /// Scalars are enclosed by single-quotes, which do not escaping, e.g. `key: 'value'`
    SingleQuote,
    /// Scalars are enclosed by double-quotes, and may contain escaped characters, 
    ///
    /// # Examples
    /// * `key: "value \b1998\t1999\t2000 \x0d\x0a is \r\n"`
    /// * `escaped: ' # Not a ''comment''.'`
    /// 
    /// The `EscapeFormat` indicates whether or not JSON should be supported.
    DoubleQuote(EscapeFormat),

}

impl AsRef<str> for FlowScalarStyle {
    fn as_ref(&self) -> &str {
        match *self {
            FlowScalarStyle::Plain => "",
            FlowScalarStyle::SingleQuote => "'",
            FlowScalarStyle::DoubleQuote(_) => "\""
        }
    }
}

/// There are two groups of styles. Block styles use indentation to denote structure; In contrast,
/// flow styles styles rely on explicit indicators.
///
/// [YAML Spec](http://www.yaml.org/spec/1.2/spec.html#id2766446)
#[derive(Debug, PartialEq, Clone)]
pub enum StructureStyle {
    /// Uses indentation to denote structure
    Block,
    /// Uses explicit indicators. This forces all child-structures to use the 
    /// `Flow` style as well.
    Flow,
}


/// Specify how scalars are serialized.
///
/// [YAML Spec](http://www.yaml.org/spec/1.2/spec.html#id2766446)
#[derive(Debug, PartialEq, Clone)]
pub enum ScalarStyle {
    /// Use indentation to denote scalar values
    Block(BlockScalarStyle),

    /// Use Indicators to mark the start and end of a possibly multi-line 
    /// scalar value. Whether or not line-breaks are inserted depends on the 
    /// `multiline_scalar_maximum_width_hint` setting.
    /// 
    /// The *first tuple struct member* is the maximum width hint of a scalar line within
    /// a scalar value. If the scalar line length exceeds this value, we may break it onto
    /// a new line.
    /// It is a hint only, and if too small, some
    /// scalar lines might end up with greater width than specified here.
    ///
    /// The *second tuple struct member* identifies the style of the scalars in flow style.
    Flow(usize, FlowScalarStyle)
}


/// Identifies the character encoding to use.
///
/// [YAML Spec](http://www.yaml.org/spec/1.2/spec.html#id2771184)
#[derive(Debug, PartialEq, Clone)]
pub enum Encoding {
    /// See the [WIKI entry](http://en.wikipedia.org/wiki/UTF-8) for details
    /// We will write a BOM at the beginning of 
    Utf8(Option<ByteOrderMark>),
}

impl AsRef<[u8]> for Encoding {
    /// Convert ourselves to the ByteOrderMark, or "" if there is no BOM
    fn as_ref(&self) -> &[u8] {
        match *self {
            Encoding::Utf8(Some(_)) 
                => b"\xEF\xBB\xBF",
            Encoding::Utf8(None)
                => b"",
        }
    }
}

/// A tag is meta-data to provide information about a value.
///
/// Built-in tags are specified using the `!!<tag>` notation within a YAML stream,
/// and are mapped to standard Rust datatypes by the implementation. In theory,
/// the application can define custom tags to help dealing with any data type.
///
/// [YAML Spec](http://www.yaml.org/spec/1.2/spec.html#id2764295)
#[derive(Clone, PartialEq, Eq)]
pub enum Tag {
    Null,
    Bool,
    Int,
    Float,
    Str,
    Map,
    Seq,
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        match *self {
            Tag::Null => "!!null",
            Tag::Bool => "!!bool",
            Tag::Int => "!!int",
            Tag::Float => "!!float",
            Tag::Str  => "!!str",
            Tag::Map  => "!!map",
            Tag::Seq  => "!!seq",
        }
    }
}

/// A marker to identify whether or not a byte order mark should be used
#[derive(Debug, PartialEq, Clone)]
pub struct ByteOrderMark;

impl Default for Encoding {
    fn default() -> Self {
        Encoding::Utf8(None)
    }
}

/// Combines all information necessary to serialize a structure, like mappings 
/// and sequences.
#[derive(Debug, PartialEq, Clone)]
pub struct StructureDetails {
    pub style: StructureStyle,
    /// If true, a `!!tag` will be serialized even though an implicit one would do as well.
    pub explicit_tag: bool,
}

/// Contains all information to describe how to serialize mappings in YAML.
#[derive(Debug, PartialEq, Clone)]
pub struct MappingDetails {
    pub details: StructureDetails,
    /// If true, use an explicit form to describe keys and values in a mapping, for all 
    /// keys and values.
    /// If false, it will only be used if the mapping key is a non-scalar one.
    /// 
    /// * [YAML Spec: block entry](http://www.yaml.org/spec/1.2/spec.html#id2798425)
    /// * [YAML Spec: single pair](http://www.yaml.org/spec/1.2/spec.html#id2792424)
    pub explicit_entries: bool,

    /// Determine how null values are presented
    pub null_style: NullScalarStyle
}

/// Determine how null values are represented in various contexts
#[derive(Debug, PartialEq, Clone)]
pub enum NullScalarStyle {
    /// Always show null values as `null`.
    /// It is affected by the settings for `scalar_value_details`
    Show,
    /// Hide null values, if they are in an entry, or standalone
    HideValue,
    /// Hide the entire entry of a mapping, i.e. key: null, or if they are standalone
    HideEntry,
}

/// Combines all information necessary to serialize a scalar, like keys or values
#[derive(Debug, PartialEq, Clone)]
pub struct ScalarDetails {
    pub style: ScalarStyle,
    /// If true, a `!!tag` will be serialized even though an implicit one would do as well.
    pub explicit_tag: bool,
}

/// A marker to signal that the YAML directive should be produced at the beginning of the stream.
///
/// [YAML Spec](http://www.yaml.org/spec/1.2/spec.html#id2781553)
#[derive(Debug, PartialEq, Clone)]
pub struct YamlVersionDirective;

impl AsRef<str> for YamlVersionDirective {
    fn as_ref(&self) -> &str {
        "%YAML 1.2"
    }
}

/// Specifies how to separate various documents in a YAML stream.
///
/// [YAML Spec](http://www.yaml.org/spec/1.2/spec.html#id2800132)
#[derive(Debug, PartialEq, Clone)]
pub enum DocumentIndicatorStyle {
    /// Enforce showing a document start indicator `---`, even in single-document mode 
    /// (i.e. `dump(...)`.
    /// In multi-document mode, i.e. `dump_all(...)`, the start indicator is automatically 
    /// used as it is a requirement.
    ///
    /// If the start of document marker is used, the YAML version directive may be specified.
    Start(Option<YamlVersionDirective>),
    /// Enforce showing the start `---` and end `...` of document indicator for each 
    /// dumped document. The behavior is similar in both `dump(...)` and `dump_all(...)` modes.
    ///
    /// If the start of document marker is used, the YAML version directive may be specified.
    StartEnd(Option<YamlVersionDirective>)
}

/// Identifies the kind of line-break characters we want to use
#[derive(Debug, PartialEq, Clone)]
pub enum LineBreak {
    LineFeed,
    CarriageReturn,
    CarriageReturnPlusLineFeed,
}

impl Default for LineBreak {
    fn default() -> Self {
        LineBreak::LineFeed
    }
}

impl AsRef<str> for LineBreak {
    fn as_ref(&self) -> &str {
        match *self {
            LineBreak::LineFeed => "\n",
            LineBreak::CarriageReturn => "\r",
            LineBreak::CarriageReturnPlusLineFeed => "\r\n",
        }
    }
}

/// Options defining how whitespace is handled within the YAML stream
#[derive(Debug, PartialEq, Clone)]
pub struct FormatDetails {
    /// Amount of spaces one indentation level will assume
    pub spaces_per_indentation_level: usize,
    /// Identifies the output encoding
    pub encoding: Encoding,
    /// Specifies the character to use for line breaks
    pub line_break: LineBreak,
}

impl Default for FormatDetails {
    fn default() -> Self {
        FormatDetails {
            spaces_per_indentation_level: 2,
            encoding: Default::default(),
            line_break: Default::default(),
        }
    }
}


/// Options to define how YAML character streams will look like.
#[derive(Debug, PartialEq, Clone)]
pub struct PresentationDetails {
    /// Defines the style for scalar values, like strings, e.g. `key: string_value`
    /// that serialize to a string shorter than the `small_scalar_string_value_width_threshold`
    pub small_scalar_string_value_details: ScalarDetails,
    /// Defines the style for scalar values, like strings, that are not considered
    /// small as their string width is larger than the 
    /// given `small_scalar_string_value_width_threshold`.
    pub big_scalar_string_value_details: ScalarDetails,
    /// If the serialized string of a scalar value is smaller than this one, they
    /// `small_scalar_string_value_details` is applied, otherwise it will 
    /// be the `big_scalar_string_value_details`
    pub small_scalar_string_value_width_threshold: usize,
    /// Specifies how keys in mappings are serialized.
    pub scalar_key_details: ScalarDetails,
    /// Specifies how non-strings scalars, like null and numbers, should be presented
    pub scalar_value_details: ScalarDetails,
    /// Defines the details for sequences, e.g. lists and tuples
    pub sequence_details: StructureDetails,
    /// Defines the details of mappings, e.g. structures and HashMaps
    pub mapping_details: MappingDetails,
    /// Specify how documents should be marked.
    ///
    /// If `None`, we will not show any document indicators in single-document mode,
    /// but show document start indicators (`---`) in multi-document mode between
    /// the documents only.
    ///
    /// If `Some(...)` is used, we will enforce a particular indicator style.
    pub document_indicator_style: Option<DocumentIndicatorStyle>,
    /// Specify how to output characters and how to deal with line-breaks
    pub format: FormatDetails,
}

impl Default for PresentationDetails {
    /// Standard YAML style, as human-readable as possible
    fn default() -> Self {
        PresentationDetails {
            small_scalar_string_value_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::Plain),
                explicit_tag: false
            },
            big_scalar_string_value_details: ScalarDetails {
                style: ScalarStyle::Block(BlockScalarStyle::Literal),
                explicit_tag: false
            },
            small_scalar_string_value_width_threshold: 20,
            scalar_key_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::Plain),
                explicit_tag: false
            },
            scalar_value_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::Plain),
                explicit_tag: false
            },
            sequence_details: StructureDetails{
                style: StructureStyle::Block,
                explicit_tag: false,
            },
            mapping_details: MappingDetails {
                details: StructureDetails{
                    style: StructureStyle::Block,
                    explicit_tag: false,
                },
                explicit_entries: false,
                null_style: NullScalarStyle::HideValue,
            },
            document_indicator_style: None,
            format: Default::default()
        }
    }
}

impl PresentationDetails {
    /// Convenience method for completeness, returning details producing a human-readable 
    /// YAML document
    pub fn yaml() -> PresentationDetails {
        Default::default()
    }

    /// Returns PresentationDetails which produce a document compatible with the JSON format.
    /// 
    /// * **valid JSON is valid YAML**
    ///   - This works because JSON is a valid subset of YAML
    /// * **Warning**
    ///   - Depending on your data structures, the serializer might insert 
    ///     Tags to further specify the underlying type, which would yield a document which 
    ///     doesn't satisfy JSON requirements.
    /// * **No multi-document support**
    ///   - As JSON only provides a single namespace for documents, valid multi-document JSON
    ///     files cannot be generated. Instead, consider serializing multiple documents as list 
    ///     in a single document.
    pub fn json() -> PresentationDetails {
        PresentationDetails {
            small_scalar_string_value_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote(EscapeFormat::JSON)),
                explicit_tag: false
            },
            big_scalar_string_value_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote(EscapeFormat::JSON)),
                explicit_tag: false
            },
            small_scalar_string_value_width_threshold: 20,         // doesn't matter
            scalar_key_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote(EscapeFormat::JSON)),
                explicit_tag: false
            },
            scalar_value_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::Plain),
                explicit_tag: false
            },
            sequence_details: StructureDetails{
                style: StructureStyle::Flow,
                explicit_tag: false,
            },
            mapping_details: MappingDetails {
                details: StructureDetails{
                    style: StructureStyle::Flow,
                    explicit_tag: false,
                },
                explicit_entries: false,
                null_style: NullScalarStyle::Show,
            },
            document_indicator_style: None,
            format: Default::default()
        }
    }


    /// Produces details that style a YAML document into a canonical form, that is a form
    /// which makes all types explicit, easing comparison
    pub fn canonical() -> PresentationDetails {
        PresentationDetails {
            small_scalar_string_value_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote(EscapeFormat::YAML)),
                explicit_tag: true
            },
            big_scalar_string_value_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote(EscapeFormat::YAML)),
                explicit_tag: true
            },
            small_scalar_string_value_width_threshold: 20,
            scalar_key_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote(EscapeFormat::YAML)),
                explicit_tag: true
            },
            scalar_value_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote(EscapeFormat::YAML)),
                explicit_tag: true
            },
            sequence_details: StructureDetails{
                style: StructureStyle::Flow,
                explicit_tag: true,
            },
            mapping_details: MappingDetails { 
                details: StructureDetails {
                    style: StructureStyle::Flow,
                    explicit_tag: true,
                },
                explicit_entries: true,
                null_style: NullScalarStyle::Show,
            },
            document_indicator_style: Some(DocumentIndicatorStyle::Start(Some(YamlVersionDirective))),
            format: FormatDetails {
                spaces_per_indentation_level: 2,
                encoding: Default::default(),
                line_break: Default::default(),
            }
        }
    }
}
