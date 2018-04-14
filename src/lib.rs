extern crate serde;
#[cfg(test)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

mod error;
mod ser;
mod de;

pub use error::{Error, ErrorKind, Result};
pub use ser::{to_string, Serializer};
//pub use de::{from_str, Deserializer};