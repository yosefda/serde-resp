use serde::ser::{self, Serialize, Impossible};
use error::{Error, ErrorKind, Result};
use std::fmt::Display;

pub struct Serializer {
    output: String,
}

pub fn to_string<T>(value: &T) -> Result<String> where T: Serialize {
    let mut serializer = Serializer { output: String::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_bool(self, v: bool) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_bool".to_owned()).into())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_i8".to_owned()).into())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_i16".to_owned()).into())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_i32".to_owned()).into())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_i64".to_owned()).into())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_u8".to_owned()).into())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_u16".to_owned()).into())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_u32".to_owned()).into())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_u64".to_owned()).into())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_f32".to_owned()).into())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_f64".to_owned()).into())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_char".to_owned()).into())
    }

    // Serialise into bulk string.
    // Encoded into "$<string-length>\r\n<string-data>\r\n", for example "foobar" is encoded into
    // "$6\r\nfoobar\r\n".
    // Empty string is encoded into "$0\r\n\r\n".
    fn serialize_str(self, v: &str) -> Result<()> {
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("".to_owned()).into())
    }

    // @todo: null bulk string or null array
    fn serialize_none(self) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_none".to_owned()).into())
    }

    fn serialize_some<T: ? Sized>(self, value: &T) -> Result<()> where
        T: Serialize {
        Err(ErrorKind::UnsupportedOperation("serialize_some".to_owned()).into())
    }

    fn serialize_unit(self) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_unit".to_owned()).into())
    }

    fn serialize_unit_struct(self, name: &str) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_unit_struct".to_owned()).into())
    }

    fn serialize_unit_variant(self, name: &str, variant_index: u32, variant: &str) -> Result<()> {
        Err(ErrorKind::UnsupportedOperation("serialize_unit_variant".to_owned()).into())
    }

    fn serialize_newtype_struct<T: ? Sized>(self, name: &str, value: &T) -> Result<()> where
        T: Serialize {
        Err(ErrorKind::UnsupportedOperation("serialize_newtype_struct".to_owned()).into())
    }

    fn serialize_newtype_variant<T: ? Sized>(self, name: &str, variant_index: u32, variant: &str, value: &T) -> Result<()> where
        T: Serialize {
        Err(ErrorKind::UnsupportedOperation("serialize_newtype_variant".to_owned()).into())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(ErrorKind::UnsupportedOperation("serialize_seq".to_owned()).into())
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Err(ErrorKind::UnsupportedOperation("serialize_tuple".to_owned()).into())
    }

    fn serialize_tuple_struct(self, name: &str, len: usize) -> Result<(Self::SerializeStruct)> {
        Err(ErrorKind::UnsupportedOperation("serialize_tuple_struct".to_owned()).into())
    }

    fn serialize_tuple_variant(self, name: &str, variant_index: u32, variant: &str, len: usize)
        -> Result<(Self::SerializeStructVariant)> {
        Err(ErrorKind::UnsupportedOperation("serialize_tuple_variant".to_owned()).into())
    }

    fn serialize_map(self, len: Option<usize>) -> Result<(Self::SerializeMap)> {
        Err(ErrorKind::UnsupportedOperation("serialize_map".to_owned()).into())
    }

    fn serialize_struct(self, name: &str, len: usize) -> Result<(Self::SerializeStruct)> {
        Err(ErrorKind::UnsupportedOperation("serialize_struct".to_owned()).into())
    }

    fn serialize_struct_variant(self, name: &str, variant_index: u32, variant: &str, len: usize)
        -> Result<(Self::SerializeStructVariant)> {
        Err(ErrorKind::UnsupportedOperation("serialize_struct_variant".to_owned()).into())
    }

    fn collect_str<T: ? Sized>(self, value: &T) -> Result<()> where
        T: Display {
        Err(ErrorKind::UnsupportedOperation("collect_str".to_owned()).into())
    }

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;
}
