use std::io;
use std::num::FpCategory;
use std::string::FromUtf8Error;

use serde::ser;

/// The only allowed indentation character
const INDENT: &'static [u8] = b" ";

/// A structure for implementing serialization to JSON.
pub struct Serializer<W, F> {
    writer: W,
    formatter: F,

    /// `first` is used to signify if we should print a comma when we are walking through a
    /// sequence.
    first: bool,
}

impl<W> Serializer<W, StandardFormatter>
    where W: io::Write,
{
    /// Creates a new YAML serializer.
    #[inline]
    pub fn new(writer: W) -> Self {
        Serializer::with_formatter(writer, StandardFormatter::new())
    }
}


impl<W, F> Serializer<W, F>
    where W: io::Write,
          F: Formatter,
{
    /// Creates a new YAML visitor whose output will be written to the writer
    /// specified.
    #[inline]
    pub fn with_formatter(writer: W, formatter: F) -> Self {
        Serializer {
            writer: writer,
            formatter: formatter,
            first: false,
        }
    }

    /// Unwrap the `Writer` from the `Serializer`.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W, F> ser::Serializer for Serializer<W, F>
    where W: io::Write,
          F: Formatter,
{
    type Error = io::Error;

    #[inline]
    fn visit_bool(&mut self, value: bool) -> io::Result<()> {
        if value {
            self.writer.write_all(b"true")
        } else {
            self.writer.write_all(b"false")
        }
    }

    #[inline]
    fn visit_isize(&mut self, value: isize) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    #[inline]
    fn visit_i8(&mut self, value: i8) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    #[inline]
    fn visit_i16(&mut self, value: i16) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    #[inline]
    fn visit_i32(&mut self, value: i32) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    #[inline]
    fn visit_i64(&mut self, value: i64) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    #[inline]
    fn visit_usize(&mut self, value: usize) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    #[inline]
    fn visit_u8(&mut self, value: u8) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    #[inline]
    fn visit_u16(&mut self, value: u16) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    #[inline]
    fn visit_u32(&mut self, value: u32) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    #[inline]
    fn visit_u64(&mut self, value: u64) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    #[inline]
    fn visit_f32(&mut self, value: f32) -> io::Result<()> {
        fmt_f32_or_null(&mut self.writer, value)
    }

    #[inline]
    fn visit_f64(&mut self, value: f64) -> io::Result<()> {
        fmt_f64_or_null(&mut self.writer, value)
    }

    #[inline]
    fn visit_char(&mut self, value: char) -> io::Result<()> {
        escape_char(&mut self.writer, value)
    }

    #[inline]
    fn visit_str(&mut self, value: &str) -> io::Result<()> {
        escape_str(&mut self.writer, value)
    }

    #[inline]
    fn visit_none(&mut self) -> io::Result<()> {
        self.visit_unit()
    }

    #[inline]
    fn visit_some<V>(&mut self, value: V) -> io::Result<()>
        where V: ser::Serialize
    {
        value.serialize(self)
    }

    #[inline]
    fn visit_unit(&mut self) -> io::Result<()> {
        self.writer.write_all(b"null")
    }

    #[inline]
    fn visit_enum_unit(&mut self, _name: &str, variant: &str) -> io::Result<()> {
        try!(self.formatter.open(&mut self.writer, b'{'));
        try!(self.formatter.comma(&mut self.writer, true));
        try!(self.visit_str(variant));
        try!(self.formatter.colon(&mut self.writer));
        try!(self.writer.write_all(b"[]"));
        self.formatter.close(&mut self.writer, b'}')
    }

    #[inline]
    fn visit_seq<V>(&mut self, mut visitor: V) -> io::Result<()>
        where V: ser::SeqVisitor,
    {
        match visitor.len() {
            Some(len) if len == 0 => {
                self.writer.write_all(b"[]")
            }
            _ => {
                try!(self.formatter.open(&mut self.writer, b'['));

                self.first = true;

                while let Some(()) = try!(visitor.visit(self)) { }

                self.formatter.close(&mut self.writer, b']')
            }
        }

    }

    #[inline]
    fn visit_enum_seq<V>(&mut self, _name: &str, variant: &str, visitor: V) -> io::Result<()>
        where V: ser::SeqVisitor,
    {
        try!(self.formatter.open(&mut self.writer, b'{'));
        try!(self.formatter.comma(&mut self.writer, true));
        try!(self.visit_str(variant));
        try!(self.formatter.colon(&mut self.writer));
        try!(self.visit_seq(visitor));
        self.formatter.close(&mut self.writer, b'}')
    }

    #[inline]
    fn visit_seq_elt<T>(&mut self, value: T) -> io::Result<()>
        where T: ser::Serialize,
    {
        try!(self.formatter.comma(&mut self.writer, self.first));
        self.first = false;

        value.serialize(self)
    }

    #[inline]
    fn visit_map<V>(&mut self, mut visitor: V) -> io::Result<()>
        where V: ser::MapVisitor,
    {
        match visitor.len() {
            Some(len) if len == 0 => {
                self.writer.write_all(b"{}")
            }
            _ => {
                try!(self.formatter.open(&mut self.writer, b'{'));

                self.first = true;

                while let Some(()) = try!(visitor.visit(self)) { }

                self.formatter.close(&mut self.writer, b'}')
            }
        }
    }

    #[inline]
    fn visit_enum_map<V>(&mut self, _name: &str, variant: &str, visitor: V) -> io::Result<()>
        where V: ser::MapVisitor,
    {
        try!(self.formatter.open(&mut self.writer, b'{'));
        try!(self.formatter.comma(&mut self.writer, true));
        try!(self.visit_str(variant));
        try!(self.formatter.colon(&mut self.writer));
        try!(self.visit_map(visitor));

        self.formatter.close(&mut self.writer, b'}')
    }

    #[inline]
    fn visit_map_elt<K, V>(&mut self, key: K, value: V) -> io::Result<()>
        where K: ser::Serialize,
              V: ser::Serialize,
    {
        try!(self.formatter.comma(&mut self.writer, self.first));
        self.first = false;

        try!(key.serialize(self));
        try!(self.formatter.colon(&mut self.writer));
        value.serialize(self)
    }

    #[inline]
    fn format() -> &'static str {
        "json"
    }
}

/// Scalar content can be written in block notation, using a literal style (indicated by `|`)
/// where all line breaks are significant. Alternatively, they can be written with the folded 
/// style (denoted by `>`) where each line break is folded to a space unless it ends an empty or 
/// a more-indented line.
/// This style only works within a Block structure
///
/// [YAML Spec](http://www.yaml.org/spec/1.2/spec.html#style/block/)
pub enum BlockScalarStyle {
    /// Literal scalar blocks are indicated by the `|` character within the YAML file
    /// and cause line-breaks to remain significant.
    Literal,
    /// Folded scalar blocks are indicated by the `>` character within the YAML file
    /// and will translate line-breaks into spaces unless it ends with an empty or a more 
    /// indented line.
    Folded,
}

/// Defines how scalars are presented in Flow-Style. FlowStyle
/// All scalars can span multiple lines.
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
    DoubleQuote,

}

/// There are two groups of styles. Block styles use indentation to denote structure; In contrast,
/// flow styles styles rely on explicit indicators.
///
/// [YAML Spec](http://www.yaml.org/spec/1.2/spec.html#id2766446)
pub enum Style {
    /// Uses indentation to denote structure
    Block,
    /// Uses explicit indicators
    Flow,
}


/// Defines the way we shall use to preserve newlines within folded scalar block 
/// literals.
pub enum FoldedBlockScalarNewlinePreservationMode {
    /// A line break is indicated by a single blank line
    BlankLines,
    /// A line break is indicated by indenting a line by one additional level.
    ///
    /// **TODO(ST)**: is it depending on the `spaces_per_indentation_level` flag, or should this 
    /// just be hardcoded to (say) two spaces ?
    Indentation,
}

pub trait Formatter {
    fn open<W>(&mut self, writer: &mut W, ch: u8) -> io::Result<()>
        where W: io::Write;

    fn comma<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
        where W: io::Write;

    fn colon<W>(&mut self, writer: &mut W) -> io::Result<()>
        where W: io::Write;

    fn close<W>(&mut self, writer: &mut W, ch: u8) -> io::Result<()>
        where W: io::Write;
}

pub struct PresentationDetails {
    /// Amount of spaces one indentation level will assume
    pub spaces_per_indentation_level: usize,
    /// Defines the style for scalar values, like strings, e.g. `key: string_value`
    /// that serialize to a string shorter than the `small_scalar_string_value_width_threshold`
    pub small_scalar_string_value_style: Style,
    /// Defines the style for scalar values, like strings, that are not considered
    /// small as their string width is larger than the 
    /// given `small_scalar_string_value_width_threshold`.
    pub big_scalar_string_value_style: Style,
    /// If the serialized string of a scalar value is smaller than this one, they
    /// `small_scalar_string_value_style` is applied, otherwise it will 
    /// be the `big_scalar_string_value_style`
    pub small_scalar_string_value_width_threshold: usize,
    /// Defines the style to use for all scalar values which are presented in Flow style
    pub flow_scalar_value_style: FlowScalarStyle,
    /// Defines the style to use for all scalar values which are presented in Block style
    pub block_scalar_value_style: BlockScalarStyle,
    /// Defines the style for sequences, e.g. lists and tuples
    pub sequence_style: Style,
    /// Defines the style of mappings, e.g. structures and HashMaps
    pub mapping_style: Style,
    /// Enforces a `---` indicator at the start of the document, even if `dump()` is used
    pub explicit_document_start_indicator: bool,
    /// Enforces a `...` indicator at the end of the document, even if `dump()` is used.
    /// In case streaming documents, you would want to set this to true, to enforce an explicit 
    /// end-of-document indicator as well.
    pub explicit_document_end_indicator: bool,
    /// The maximum width of a folded scalar line. It is a hint only, and if too small, some
    /// scalar lines might end up with greater width than specified here.
    pub folded_block_scalar_maximum_width_hint: usize,
    /// Defines how newlines are indicated for folded block scalars
    pub folded_block_scalar_newline_preservation_mode: FoldedBlockScalarNewlinePreservationMode,
}

impl Default for PresentationDetails {
    /// Standard YAML style, as human-readable as possible
    fn default() -> Self {
        PresentationDetails {
            spaces_per_indentation_level: 2,
            small_scalar_string_value_style: Style::Flow,
            big_scalar_string_value_style: Style::Block,
            small_scalar_string_value_width_threshold: 20,
            flow_scalar_value_style: FlowScalarStyle::Plain,
            block_scalar_value_style: BlockScalarStyle::Literal,
            sequence_style: Style::Block,
            mapping_style: Style::Block,
            explicit_document_start_indicator: false,
            explicit_document_end_indicator: false,
            folded_block_scalar_maximum_width_hint: 0,
            folded_block_scalar_newline_preservation_mode:
                                        FoldedBlockScalarNewlinePreservationMode::Indentation
        }
    }
}

impl PresentationDetails {
    /// Convenience method for completeness, returning details producing a human-readable 
    /// YAML document
    fn yaml() -> PresentationDetails {
        Default::default()
    }

    /// Returns PresentationDetails which produce a document compatible with the JSON format.
    /// 
    /// *NOTE*: This works because JSON is a valid subset of YAML
    /// **Warning**: Depending on your data structures, the serializer might insert 
    /// Tags to further specify the underlying type, which would yield a document which 
    /// doesn't satisfy JSON requirements.
    fn json() -> PresentationDetails {
        PresentationDetails {
            spaces_per_indentation_level: 2,
            small_scalar_string_value_style: Style::Flow,
            big_scalar_string_value_style: Style::Flow,
            small_scalar_string_value_width_threshold: 20,         // doesn't matter
            flow_scalar_value_style: FlowScalarStyle::DoubleQuote,
            block_scalar_value_style: BlockScalarStyle::Literal,   // doesn't matter
            sequence_style: Style::Flow,
            mapping_style: Style::Flow,
            explicit_document_start_indicator: false,
            explicit_document_end_indicator: false,
            folded_block_scalar_maximum_width_hint: 0,              // doesn't matter
            folded_block_scalar_newline_preservation_mode:          // doesn't matter
                                        FoldedBlockScalarNewlinePreservationMode::Indentation
        }
    }
}

pub struct StandardFormatter {
    current_indent: usize,
    opts: PresentationDetails,
}

impl StandardFormatter {
    fn new() -> Self {
        StandardFormatter::with_options(Default::default())
    }

    fn with_options(options: PresentationDetails) -> Self {
        StandardFormatter {
            current_indent: 0,
            opts: options,
        }
    }
}

impl Formatter for StandardFormatter {
    fn open<W>(&mut self, writer: &mut W, ch: u8) -> io::Result<()>
        where W: io::Write,
    {
        self.current_indent += 1;
        writer.write_all(&[ch])
    }

    fn comma<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
        where W: io::Write,
    {
        if first {
            try!(writer.write_all(b"\n"));
        } else {
            try!(writer.write_all(b",\n"));
        }

        indent(writer, self.current_indent * self.opts.spaces_per_indentation_level)
    }

    fn colon<W>(&mut self, writer: &mut W) -> io::Result<()>
        where W: io::Write,
    {
        writer.write_all(b": ")
    }

    fn close<W>(&mut self, writer: &mut W, ch: u8) -> io::Result<()>
        where W: io::Write,
    {
        self.current_indent -= 1;
        try!(writer.write(b"\n"));
        try!(indent(writer, self.current_indent * self.opts.spaces_per_indentation_level));

        writer.write_all(&[ch])
    }
}

#[inline]
pub fn escape_bytes<W>(wr: &mut W, bytes: &[u8]) -> io::Result<()>
    where W: io::Write
{
    try!(wr.write_all(b"\""));

    let mut start = 0;

    for (i, byte) in bytes.iter().enumerate() {
        let escaped = match *byte {
            b'"' => b"\\\"",
            b'\\' => b"\\\\",
            b'\x08' => b"\\b",
            b'\x0c' => b"\\f",
            b'\n' => b"\\n",
            b'\r' => b"\\r",
            b'\t' => b"\\t",
            _ => { continue; }
        };

        if start < i {
            try!(wr.write_all(&bytes[start..i]));
        }

        try!(wr.write_all(escaped));

        start = i + 1;
    }

    if start != bytes.len() {
        try!(wr.write_all(&bytes[start..]));
    }

    try!(wr.write_all(b"\""));
    Ok(())
}

#[inline]
pub fn escape_str<W>(wr: &mut W, value: &str) -> io::Result<()>
    where W: io::Write
{
    escape_bytes(wr, value.as_bytes())
}

#[inline]
fn escape_char<W>(wr: &mut W, value: char) -> io::Result<()>
    where W: io::Write
{
    // FIXME: this allocation is required in order to be compatible with stable
    // rust, which doesn't support encoding a `char` into a stack buffer.
    escape_bytes(wr, value.to_string().as_bytes())
}

fn fmt_f32_or_null<W>(wr: &mut W, value: f32) -> io::Result<()>
    where W: io::Write
{
    match value.classify() {
        FpCategory::Nan | FpCategory::Infinite => wr.write_all(b"null"),
        _ => {
            let s = format!("{:?}", value);
            try!(wr.write_all(s.as_bytes()));
            if !s.contains('.') {
                try!(wr.write_all(b".0"))
            }
            Ok(())
        }
    }
}

fn fmt_f64_or_null<W>(wr: &mut W, value: f64) -> io::Result<()>
    where W: io::Write
{
    match value.classify() {
        FpCategory::Nan | FpCategory::Infinite => wr.write_all(b"null"),
        _ => {
            let s = format!("{:?}", value);
            try!(wr.write_all(s.as_bytes()));
            if !s.contains('.') {
                try!(wr.write_all(b".0"))
            }
            Ok(())
        }
    }
}

/// Encode the specified struct into a json `[u8]` writer.
#[inline]
pub fn to_writer<W, T>(writer: &mut W, value: &T) -> io::Result<()>
    where W: io::Write,
          T: ser::Serialize,
{
    let mut ser = Serializer::new(writer);
    try!(value.serialize(&mut ser));
    Ok(())
}

/// Encode the specified struct into a json `[u8]` buffer.
#[inline]
pub fn to_vec<T>(value: &T) -> Vec<u8>
    where T: ser::Serialize,
{
    // We are writing to a Vec, which doesn't fail. So we can ignore
    // the error.
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value).unwrap();
    writer
}

/// Encode the specified struct into a json `String` buffer.
#[inline]
pub fn to_string<T>(value: &T) -> Result<String, FromUtf8Error>
    where T: ser::Serialize
{
    let vec = to_vec(value);
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
