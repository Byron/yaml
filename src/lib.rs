//! YAML serialization and deserialization.
//!
//! It follows the [YAML Spec 1.2][yaml-spec].
//! 
//! # What is YAML
//! 
//! TODO
//! 
//! # Limitations
//! 
//! This paragraph contains the differences of the implementation compared to 
//! the requirements of the underlying [yaml specification][yaml-spec].
//! 
//! * NO SERIALZATION SUPPORT
//! * NO DESERIALIZTION SUPPORT
//! 
//! [yaml-spec]: http://www.yaml.org/spec/1.2/spec.html
extern crate serde;
extern crate num;

pub mod ser;

pub use ser::{to_writer, to_writer_with_options};
pub use ser::{to_string, to_string_with_options};
pub use ser::{to_vec, to_vec_with_options};
