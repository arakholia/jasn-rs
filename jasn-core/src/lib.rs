//! Core data types for JASN and JAML serialization formats.
//!
//! This crate provides the shared data model used by both JASN (JSON5-like) and JAML (YAML-like)
//! serialization formats. Both formats share the same type system and in-memory representation.
//!
//! # Data Model
//!
//! ```rust
//! use std::collections::BTreeMap;
//! use jasn_core::Value;
//!
//! # fn main() {
//! let mut map = BTreeMap::new();
//! map.insert("name".to_string(), Value::String("Alice".to_string()));
//! map.insert("age".to_string(), Value::Int(30));
//!
//! let value = Value::Map(map);
//! # }
//! ```
//!
//! # Features
//!
//! - `serde` (default): Enable serde serialization/deserialization support

#![warn(missing_docs)]

mod value;
pub use value::{Binary, Timestamp, Value};

#[cfg(feature = "serde")]
pub mod de {
    //! Serde deserialization support for Value.
    pub use crate::value::de::{from_value, Error};
}

#[cfg(feature = "serde")]
pub mod ser {
    //! Serde serialization support for Value.
    pub use crate::value::ser::{to_value, Error, Serializer};
}
