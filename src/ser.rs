use std::io;
use std::num::FpCategory;
use std::string::FromUtf8Error;
use std::borrow::Borrow;

use serde::ser;

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

/// A receiver for serialization events which are translated into a stream of characters, 
/// destined to serialize value types.
pub struct Serializer<W, D = Borrow<PresentationDetails>> 
    where D: Borrow<PresentationDetails>
{
    writer: W,
    current_indent: usize,
    opts: D,
    stack: Vec<StructureKind>,

    /// `first` is used to signify if we should print a comma when we are walking through a
    /// sequence.
    first: bool,
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
            first: false,
            stack: Vec::new(),
        }
    }

    /// Unwrap the `Writer` from the `Serializer`.
    pub fn into_inner(self) -> W {
        self.writer
    }

    fn open(&mut self, kind: StructureKind) -> io::Result<()>
    {
        self.current_indent += 1;
        self.stack.push(kind.clone());
        let open_str: &[u8] = 
            match kind {
                StructureKind::Sequence => {
                    match self.opts.borrow().sequence_details.style {
                        // In block mode, we always start on a new line
                        StructureStyle::Block => self.opts.borrow().format.line_break.as_ref(),
                        StructureStyle::Flow => b"["
                    }
                },
                StructureKind::Mapping => {
                    panic!("TODO")
                }
            };

        encode_ascii(&mut self.writer, &self.opts.borrow().format.encoding, open_str)
    }

    fn comma(&mut self, first: bool) -> io::Result<()>
    {
        if first {
            try!(self.writer.write_all(b"\n"));
        } else {
            try!(self.writer.write_all(b",\n"));
        }

        indent(&mut self.writer, self.current_indent * 
               self.opts.borrow().format.spaces_per_indentation_level)
    }

    fn colon(&mut self) -> io::Result<()>
    {
        self.writer.write_all(b": ")
    }

    fn close(&mut self) -> io::Result<()>
    {
        self.current_indent -= 1;
        let kind =  self.stack.pop()
                              .expect("Calls to open() and close() must match exactly");
        try!(self.writer.write(b"\n"));
        try!(indent(&mut self.writer, self.current_indent * 
                         self.opts.borrow().format.spaces_per_indentation_level));

        // self.writer.write_all(&[ch])
        // TODO: Write actual value
        Ok(())
    }

    /// Must be called once when starting to serialize any amount of documents.
    fn begin_stream(&mut self) -> io::Result<()> {
        self.writer.write_all(self.opts.borrow().format.encoding.as_ref())
    }

    /// Must be called once before starting a new document.
    fn begin_document(&mut self) -> io::Result<()> {
        match self.opts.borrow().document_indicator_style {
             Some(DocumentIndicatorStyle::Start(ref yaml_directive))
            |Some(DocumentIndicatorStyle::StartEnd(ref yaml_directive)) => {
                // We don't care about the actual type of it, as we support single-document only
                // TODO(ST) review this once multi-document modes are possible
                try!(
                    match *yaml_directive {
                        Some(ref yaml_directive) => {
                            try!(encode_ascii(&mut self.writer, 
                                              &self.opts.borrow().format.encoding,
                                              yaml_directive));
                            // need line-break after version directive
                            encode_ascii(&mut self.writer, &self.opts.borrow().format.encoding, 
                                         &self.opts.borrow().format.line_break)
                        },
                        None 
                            => Ok(())
                    });

                // We may assume that if called, there is at least one value coming.
                // Therefore we can put a space here by default, as values will not 
                // take care of that
                encode_ascii(&mut self.writer, &self.opts.borrow().format.encoding, b"---")
            },
            None => Ok(()),
        }
    }

    /// The sibling of `begin_document()`, which must be called exactly once.
    fn end_document(&mut self) -> io::Result<()> {
        match self.opts.borrow().document_indicator_style {
            Some(DocumentIndicatorStyle::StartEnd(_)) => {
                    try!(encode_ascii(&mut self.writer, &self.opts.borrow().format.encoding,
                                      &self.opts.borrow().format.line_break));
                    encode_ascii(&mut self.writer, &self.opts.borrow().format.encoding, b"...")
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
                if let ScalarStyle::Flow(ref width, ref flow_style) = opts.style {
                    opts.explicit_tag = 
                        match *flow_style {
                            FlowScalarStyle::Plain => opts.explicit_tag,
                            _ => true
                        };
                }
                false
            };

        if opts.explicit_tag {
            try!(encode_ascii(&mut self.writer, encoding, b" "));
            try!(encode_ascii(&mut self.writer, encoding, tag.as_ref()));
        }

        match opts.style {
            ScalarStyle::Block(_) => panic!("TODO"),
            ScalarStyle::Flow(ref width, ref flow_style) => {
                let (flow_style, str_slice) = 
                    if have_string {
                        let str_slice = 
                            if *width == 0 {
                                chars.as_ref()
                            } else {
                                panic!("TODO: Folding within a flow string")
                            };
                        panic!("TODO: escape handling! if there is something to escape, we must use \"")
                    } else {
                        // Any other scalars neither need folding, nor do they need escaping
                        (flow_style, chars.as_ref())
                    };


                // TODO(ST): deal with indentation/line breaks
                try!(encode_ascii(&mut self.writer, encoding, b" "));
                try!(encode_ascii(&mut self.writer, encoding, flow_style));
                try!(encode_str(&mut self.writer, encoding, str_slice));
                encode_ascii(&mut self.writer, &self.opts.borrow().format.encoding, flow_style)
            },
        }
    }
}

impl<W, D> ser::Serializer for Serializer<W, D>
    where W: io::Write,
          D: Borrow<PresentationDetails>
{
    type Error = io::Error;

    fn visit_bool(&mut self, value: bool) -> io::Result<()> {
        if value {
            self.writer.write_all(b"true")
        } else {
            self.writer.write_all(b"false")
        }
    }

    fn visit_isize(&mut self, value: isize) -> io::Result<()> {
        write!(self.writer, "{}", value)
    }

    fn visit_i8(&mut self, value: i8) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    fn visit_i16(&mut self, value: i16) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    fn visit_i32(&mut self, value: i32) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    fn visit_i64(&mut self, value: i64) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    fn visit_usize(&mut self, value: usize) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    fn visit_u8(&mut self, value: u8) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    fn visit_u16(&mut self, value: u16) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    fn visit_u32(&mut self, value: u32) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    fn visit_u64(&mut self, value: u64) -> io::Result<()> {
        write!(&mut self.writer, "{}", value)
    }

    fn visit_f32(&mut self, value: f32) -> io::Result<()> {
        fmt_f32_or_null(&mut self.writer, value)
    }

    fn visit_f64(&mut self, value: f64) -> io::Result<()> {
        fmt_f64_or_null(&mut self.writer, value)
    }

    fn visit_char(&mut self, value: char) -> io::Result<()> {
        escape_char(&mut self.writer, value)
    }

    fn visit_str(&mut self, value: &str) -> io::Result<()> {
        escape_str(&mut self.writer, value)
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
        match self.opts.borrow().mapping_details.null_style {
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
        try!(self.open(StructureKind::Mapping));
        try!(self.comma(true));
        try!(self.visit_str(variant));
        try!(self.colon());
        try!(self.writer.write_all(b"[]"));
        self.close()
    }

    fn visit_seq<V>(&mut self, mut visitor: V) -> io::Result<()>
        where V: ser::SeqVisitor,
    {
        match visitor.len() {
            Some(len) if len == 0 => {
                self.writer.write_all(b"[]")
            }
            _ => {
                try!(self.open(StructureKind::Sequence));

                self.first = true;

                while let Some(()) = try!(visitor.visit(self)) { }

                self.close()
            }
        }

    }

    fn visit_enum_seq<V>(&mut self, _name: &str, variant: &str, visitor: V) -> io::Result<()>
        where V: ser::SeqVisitor,
    {
        try!(self.open(StructureKind::Mapping));
        try!(self.comma(true));
        try!(self.visit_str(variant));
        try!(self.colon());
        try!(self.visit_seq(visitor));
        self.close()
    }

    fn visit_seq_elt<T>(&mut self, value: T) -> io::Result<()>
        where T: ser::Serialize,
    {
        let first = self.first;
        try!(self.comma(first));
        self.first = false;

        value.serialize(self)
    }

    fn visit_map<V>(&mut self, mut visitor: V) -> io::Result<()>
        where V: ser::MapVisitor,
    {
        match visitor.len() {
            Some(len) if len == 0 => {
                self.writer.write_all(b"{}")
            }
            _ => {
                try!(self.open(StructureKind::Mapping));

                self.first = true;

                while let Some(()) = try!(visitor.visit(self)) { }

                self.close()
            }
        }
    }

    fn visit_enum_map<V>(&mut self, _name: &str, variant: &str, visitor: V) -> io::Result<()>
        where V: ser::MapVisitor,
    {
        try!(self.open(StructureKind::Mapping));
        try!(self.comma(true));
        try!(self.visit_str(variant));
        try!(self.colon());
        try!(self.visit_map(visitor));

        self.close()
    }

    fn visit_map_elt<K, V>(&mut self, key: K, value: V) -> io::Result<()>
        where K: ser::Serialize,
              V: ser::Serialize,
    {
        let first = self.first;
        try!(self.comma(first));
        self.first = false;

        try!(key.serialize(self));
        try!(self.colon());
        value.serialize(self)
    }

    fn format() -> &'static str {
        "yaml"
    }
}

fn escape_bytes<W>(wr: &mut W, bytes: &[u8]) -> io::Result<()>
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

fn escape_str<W>(wr: &mut W, value: &str) -> io::Result<()>
    where W: io::Write
{
    escape_bytes(wr, value.as_bytes())
}

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

fn encode_ascii<W, B>(writer: &mut W, encoding: &Encoding, chars: B) -> io::Result<()> 
    where W: io::Write,
          B: AsRef<[u8]>
{
    if chars.as_ref().len() == 0 {
        return Ok(())
    }
    
    match *encoding {
        // ASCII is a valid subset of UTF8, and can thus be written directly
        Encoding::Utf8(_) => writer.write_all(chars.as_ref()),
    }
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
    DoubleQuote,

}

impl AsRef<[u8]> for FlowScalarStyle {
    fn as_ref(&self) -> &[u8] {
        match *self {
            FlowScalarStyle::Plain => b"",
            FlowScalarStyle::SingleQuote => b"'",
            FlowScalarStyle::DoubleQuote => b"\""
        }
    }
}

/// There are two groups of styles. Block styles use indentation to denote structure; In contrast,
/// flow styles styles rely on explicit indicators.
///
/// [YAML Spec](http://www.yaml.org/spec/1.2/spec.html#id2766446)
#[derive(Debug, PartialEq)]
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
    /// Convert ourselves to the ByteOrderMark, or b"" if there is no BOM
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
pub enum Tag {
    Null,
    Str,
}

impl AsRef<[u8]> for Tag {
    fn as_ref(&self) -> &[u8] {
        match *self {
            Tag::Null => b"!!null",
            Tag::Str  => b"!!str",
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
#[derive(Debug, PartialEq)]
pub struct StructureDetails {
    pub style: StructureStyle,
    /// If true, a `!!tag` will be serialized even though an implicit one would do as well.
    pub explicit_tag: bool,
}

/// Contains all information to describe how to serialize mappings in YAML.
#[derive(Debug, PartialEq)]
pub struct MappingDetails {
    pub details: StructureDetails,
    /// If true, use an explicit form to describe keys and values in a mapping, for all 
    /// keys and values.
    /// If false, it will only be used if the mapping key is a non-scalar one.
    /// 
    /// [YAML Spec](http://www.yaml.org/spec/1.2/spec.html#id2798425)
    pub explicit_block_entries: bool,

    /// Determine how null values are presented
    pub null_style: NullScalarStyle
}

/// Determine how null values are represented in various contexts
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub struct YamlVersionDirective;

impl AsRef<[u8]> for YamlVersionDirective {
    fn as_ref(&self) -> &[u8] {
        b"%YAML 1.2"
    }
}

/// Specifies how to separate various documents in a YAML stream.
///
/// [YAML Spec](http://www.yaml.org/spec/1.2/spec.html#id2800132)
#[derive(Debug, PartialEq)]
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

impl AsRef<[u8]> for LineBreak {
    fn as_ref(&self) -> &[u8] {
        match *self {
            LineBreak::LineFeed => b"\n",
            LineBreak::CarriageReturn => b"\r",
            LineBreak::CarriageReturnPlusLineFeed => b"\r\n",
        }
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
#[derive(Debug, PartialEq)]
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
                explicit_block_entries: false,
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
                style: ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote),
                explicit_tag: false
            },
            big_scalar_string_value_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote),
                explicit_tag: false
            },
            small_scalar_string_value_width_threshold: 20,         // doesn't matter
            scalar_key_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote),
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
                explicit_block_entries: false,
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
                style: ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote),
                explicit_tag: true
            },
            big_scalar_string_value_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote),
                explicit_tag: true
            },
            small_scalar_string_value_width_threshold: 20,
            scalar_key_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote),
                explicit_tag: true
            },
            scalar_value_details: ScalarDetails {
                style: ScalarStyle::Flow(0, FlowScalarStyle::DoubleQuote),
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
                explicit_block_entries: true,
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
