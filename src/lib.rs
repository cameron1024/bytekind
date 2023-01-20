#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

//! Containers for bytes that encode their serialization format as generics
//!
//! ```
//! # use bytekind::*;
//! let json = serde_json::json!([1, 2, 3, 4]);
//! let bytes: Bytes<Plain> = serde_json::from_value(json).unwrap();
//! assert_eq!(&bytes, &[1, 2, 3, 4]);
//!
//! let bytes: Bytes<HexString> = bytes.convert();
//! let hex_string = serde_json::to_string(&bytes).unwrap();
//! assert_eq!(hex_string, "\"01020304\"");
//! ```

#[cfg(feature = "std")]
mod vec;

mod array;
pub use array::ByteArray;

#[cfg(feature = "std")]
pub use vec::Bytes;

#[cfg(test)]
mod tests;

/// Types which represent a serialization format
pub trait Format {}

#[cfg(feature = "hex")]
/// Format bytes as a hex string
pub struct HexString;
#[cfg(feature = "hex")]
impl Format for HexString {}

/// Format bytes as plain bytes (i.e. what you'd get if you just `derive(Serialize)`)
pub struct Plain;
impl Format for Plain {}

