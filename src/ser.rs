use serde::ser::{self, Serialize, Impossible};
use error::{Error, ErrorKind, Result};
use std::fmt::Display;
use num_traits::Num;

pub struct Serializer {
    output: String,
}

impl Serializer {
    fn serialize_num_types<T: Num + Display>(&self, v: T) -> String {
        format!("${}\r\n{}\r\n", v.to_string().len(), v)
    }

    fn serialize_null(&self) -> String {
        format!("$-1\r\n")
    }
}

pub fn to_string<T>(value: &T) -> Result<String> where T: Serialize {
    let mut serializer = Serializer { output: String::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    // Serialise into RESP bulk strings.
    // true is encoded into "$4\r\ntrue\r\n" and false into "$5\r\nfalse\r\n".
    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        match v {
            true => self.output += &format!("$4\r\ntrue\r\n"),
            false => self.output += &format!("$5\r\nfalse\r\n"),
        }
        Ok(())
    }

    // Serialise into RESP bulk strings.
    // Encoded format is "$<number-of-bytes>\r\n<string-data>\r\n", for example "-100" is encoded
    // into "$4\r\n-100\r\n".
    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.output += &self.serialize_num_types(v);
        Ok(())
    }

    // Serialise into RESP bulk strings.
    // Encoded format is "$<number-of-bytes>\r\n<string-data>\r\n", for example "-100" is encoded
    // into "$4\r\n-100\r\n".
    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.output += &self.serialize_num_types(v);
        Ok(())
    }

    // Serialise into RESP bulk strings.
    // Encoded format is "$<number-of-bytes>\r\n<string-data>\r\n", for example "-100" is encoded
    // into "$4\r\n-100\r\n".
    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.output += &self.serialize_num_types(v);
        Ok(())
    }

    // Serialise into RESP bulk strings.
    // Encoded format is "$<number-of-bytes>\r\n<string-data>\r\n", for example "-100" is encoded
    // into "$4\r\n-100\r\n".
    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.output += &self.serialize_num_types(v);
        Ok(())
    }

    // Serialise into RESP bulk strings.
    // Encoded format is "$<number-of-bytes>\r\n<string-data>\r\n", for example "100" is encoded
    // into "$3\r\n100\r\n".
    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.output += &self.serialize_num_types(v);
        Ok(())
    }

    // Serialise into RESP bulk strings.
    // Encoded format is "$<number-of-bytes>\r\n<string-data>\r\n", for example "100" is encoded
    // into "$3\r\n100\r\n".
    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.output += &self.serialize_num_types(v);
        Ok(())
    }

    // Serialise into RESP bulk strings.
    // Encoded format is "$<number-of-bytes>\r\n<string-data>\r\n", for example "100" is encoded
    // into "$3\r\n100\r\n".
    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.output += &self.serialize_num_types(v);
        Ok(())
    }

    // Serialise into RESP bulk strings.
    // Encoded format is "$<number-of-bytes>\r\n<string-data>\r\n", for example "100" is encoded
    // into "$3\r\n100\r\n".
    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.output += &self.serialize_num_types(v);
        Ok(())
    }

    // Serialise into RESP bulk strings.
    // Encoded format is "$<number-of-bytes>\r\n<string-data>\r\n", for example "1.34" is encoded
    // into "$4\r\n1.34\r\n".
    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.output += &self.serialize_num_types(v);
        Ok(())
    }

    // Serialise into RESP bulk strings.
    // Encoded format is "$<number-of-bytes>\r\n<string-data>\r\n", for example "1.34" is encoded
    // into "$4\r\n1.34\r\n".
    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.output += &self.serialize_num_types(v);
        Ok(())
    }

    // Serialise into RESP bulk strings.
    // Encoded format is "$<number-of-bytes>\r\n<string-data>\r\n", for example "a" is encoded
    // into "$1\r\na\r\n".
    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.output += &format!("$1\r\n{}\r\n", v);
        Ok(())
    }

    // Serialise into RESP bulk string.
    // The encoded form is "$<number-of-bytes>\r\n<string-data>\r\n", for example "foobar" is
    // encoded into "$6\r\nfoobar\r\n".
    // Empty string is encoded into "$0\r\n\r\n".
    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.output += &format!("${}\r\n{}\r\n", v.len(), v);
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        Err(ErrorKind::UnsupportedOperation("".to_owned()).into())
    }

    // Serialise into RESP bulk string representation of null.
    // The encoded form is "$-1\r\n".
    fn serialize_none(self) -> Result<Self::Ok> {
        self.output += &self.serialize_null();
        Ok(())
    }

    fn serialize_some<T: ? Sized>(self, value: &T) -> Result<Self::Ok> where
        T: Serialize {
        let _result = value.serialize(self);
        Ok(())
    }

    // Serialise into RESP bulk string representation of null.
    // The encoded form is "$-1\r\n".
    fn serialize_unit(self) -> Result<Self::Ok> {
        self.output += &self.serialize_null();
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &str) -> Result<Self::Ok> {
        Err(ErrorKind::UnsupportedOperation("serialize_unit_struct".to_owned()).into())
    }

    fn serialize_unit_variant(self, _name: &str, _variant_index: u32, _variant: &str) -> Result<Self::Ok> {
        Err(ErrorKind::UnsupportedOperation("serialize_unit_variant".to_owned()).into())
    }

    fn serialize_newtype_struct<T: ? Sized>(self, _name: &str, _value: &T) -> Result<Self::Ok> where
        T: Serialize {
        Err(ErrorKind::UnsupportedOperation("serialize_newtype_struct".to_owned()).into())
    }

    fn serialize_newtype_variant<T: ? Sized>(self, _name: &str, _variant_index: u32, _variant: &str, _value: &T) -> Result<Self::Ok> where
        T: Serialize {
        Err(ErrorKind::UnsupportedOperation("serialize_newtype_variant".to_owned()).into())
    }

    // Serialise into RESP array.
    // The encoded form is "*<number-of-elements>\r\n<RESP-type-for-every-element>", for example
    // ["foo", "bar"] is encoded into "*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n".
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        // len must be known upfront.
        if len.is_none() {
            return Err(ErrorKind::SerError("sequence length is unknown".to_owned()).into());
        }
        self.output += "*";

        if len.unwrap() == 0 {
            self.output += &format!("{}\r\n", len.unwrap());
        } else {
            self.output += &format!("{}\r\n", len.unwrap());
        }

        Ok(self)
    }

    // Serialise into RESP array.
    // The encoded form is "*<number-of-elements>\r\n<RESP-type-for-every-element>", for example
    // ("foo", "bar") is encoded into "*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n".
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    // Serialise into RESP array.
    // The encoded form is "*<number-of-elements>\r\n<RESP-type-for-every-element>", for example
    // Tuple("foo", "bar") is encoded into "*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n".
    fn serialize_tuple_struct(self, _name: &str, len: usize) -> Result<(Self::SerializeTupleStruct)> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(self, _name: &str, _variant_index: u32, _variant: &str, _len: usize)
        -> Result<(Self::SerializeStructVariant)> {
        Err(ErrorKind::UnsupportedOperation("serialize_tuple_variant".to_owned()).into())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<(Self::SerializeMap)> {
        Err(ErrorKind::UnsupportedOperation("serialize_map".to_owned()).into())
    }

    fn serialize_struct(self, _name: &str, _len: usize) -> Result<(Self::SerializeStruct)> {
        Err(ErrorKind::UnsupportedOperation("serialize_struct".to_owned()).into())
    }

    fn serialize_struct_variant(self, _name: &str, _variant_index: u32, _variant: &str, _len: usize)
        -> Result<(Self::SerializeStructVariant)> {
        Err(ErrorKind::UnsupportedOperation("serialize_struct_variant".to_owned()).into())
    }

    fn collect_str<T: ? Sized>(self, _value: &T) -> Result<Self::Ok> where
        T: Display {
        Err(ErrorKind::UnsupportedOperation("collect_str".to_owned()).into())
    }

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        Ok(())
    }
}


///////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_bool() {
        assert_eq!(to_string(&true).unwrap(), "$4\r\ntrue\r\n");
        assert_eq!(to_string(&false).unwrap(), "$5\r\nfalse\r\n");
    }

    #[test]
    fn test_serialize_i8() {
        assert_eq!(to_string(&(-100 as i8)).unwrap(), "$4\r\n-100\r\n");
        assert_eq!(to_string(&(100 as i8)).unwrap(), "$3\r\n100\r\n");
    }

    #[test]
    fn test_serialize_i16() {
        assert_eq!(to_string(&(-100 as i8)).unwrap(), "$4\r\n-100\r\n");
        assert_eq!(to_string(&(100 as i8)).unwrap(), "$3\r\n100\r\n");
    }

    #[test]
    fn test_serialize_i32() {
        assert_eq!(to_string(&(-100 as i8)).unwrap(), "$4\r\n-100\r\n");
        assert_eq!(to_string(&(100 as i8)).unwrap(), "$3\r\n100\r\n");
    }

    #[test]
    fn test_serialize_i64() {
        assert_eq!(to_string(&(-100 as i8)).unwrap(), "$4\r\n-100\r\n");
        assert_eq!(to_string(&(100 as i8)).unwrap(), "$3\r\n100\r\n");
    }

    #[test]
    fn test_serialize_u8() {
        assert_eq!(to_string(&(100 as u8)).unwrap(), "$3\r\n100\r\n");
    }

    #[test]
    fn test_serialize_u16() {
        assert_eq!(to_string(&(100 as u16)).unwrap(), "$3\r\n100\r\n");
    }

    #[test]
    fn test_serialize_u32() {
        assert_eq!(to_string(&(100 as u32)).unwrap(), "$3\r\n100\r\n");
    }

    #[test]
    fn test_serialize_u64() {
        assert_eq!(to_string(&(100 as u64)).unwrap(), "$3\r\n100\r\n");
    }

    #[test]
    fn test_serialize_f32() {
        assert_eq!(to_string(&(-1.34 as f32)).unwrap(), "$5\r\n-1.34\r\n");
        assert_eq!(to_string(&(1.34 as f32)).unwrap(), "$4\r\n1.34\r\n");
    }

    #[test]
    fn test_serialize_f64() {
        assert_eq!(to_string(&(-1.34 as f64)).unwrap(), "$5\r\n-1.34\r\n");
        assert_eq!(to_string(&(1.34 as f64)).unwrap(), "$4\r\n1.34\r\n");
    }

    #[test]
    fn test_serialize_char() {
        assert_eq!(to_string(&'a').unwrap(), "$1\r\na\r\n");
    }

    #[test]
    fn test_serialize_str() {
        assert_eq!(to_string(&"").unwrap(), "$0\r\n\r\n");
        assert_eq!(to_string(&"foobar").unwrap(), "$6\r\nfoobar\r\n");
    }

    #[test]
    fn test_serialize_seq() {
        // bool
        assert_eq!(to_string(&(Vec::new() as Vec<bool>)).unwrap(), "*0\r\n");
        assert_eq!(to_string(&vec![true, false]).unwrap(), "*2\r\n$4\r\ntrue\r\n$5\r\nfalse\r\n");

        // signed int
        assert_eq!(to_string(&(Vec::new() as Vec<i8>)).unwrap(), "*0\r\n");
        assert_eq!(to_string(&vec![1 as i8, 2 as i8]).unwrap(), "*2\r\n$1\r\n1\r\n$1\r\n2\r\n");

        assert_eq!(to_string(&(Vec::new() as Vec<i16>)).unwrap(), "*0\r\n");
        assert_eq!(to_string(&vec![1 as i16, 2 as i16]).unwrap(), "*2\r\n$1\r\n1\r\n$1\r\n2\r\n");

        assert_eq!(to_string(&(Vec::new() as Vec<i32>)).unwrap(), "*0\r\n");
        assert_eq!(to_string(&vec![1 as i32, 2 as i32]).unwrap(), "*2\r\n$1\r\n1\r\n$1\r\n2\r\n");

        assert_eq!(to_string(&(Vec::new() as Vec<i64>)).unwrap(), "*0\r\n");
        assert_eq!(to_string(&vec![1 as i64, 2 as i64]).unwrap(), "*2\r\n$1\r\n1\r\n$1\r\n2\r\n");

        // unsigned int
        assert_eq!(to_string(&(Vec::new() as Vec<u8>)).unwrap(), "*0\r\n");
        assert_eq!(to_string(&vec![1 as u8, 2 as u8]).unwrap(), "*2\r\n$1\r\n1\r\n$1\r\n2\r\n");

        assert_eq!(to_string(&(Vec::new() as Vec<u16>)).unwrap(), "*0\r\n");
        assert_eq!(to_string(&vec![1 as u16, 2 as u16]).unwrap(), "*2\r\n$1\r\n1\r\n$1\r\n2\r\n");

        assert_eq!(to_string(&(Vec::new() as Vec<u32>)).unwrap(), "*0\r\n");
        assert_eq!(to_string(&vec![1 as u32, 2 as u32]).unwrap(), "*2\r\n$1\r\n1\r\n$1\r\n2\r\n");

        assert_eq!(to_string(&(Vec::new() as Vec<u64>)).unwrap(), "*0\r\n");
        assert_eq!(to_string(&vec![1 as u64, 2 as u64]).unwrap(), "*2\r\n$1\r\n1\r\n$1\r\n2\r\n");

        // float
        assert_eq!(to_string(&(Vec::new() as Vec<f32>)).unwrap(), "*0\r\n");
        assert_eq!(to_string(&vec![1 as f32, 2.14 as f32]).unwrap(), "*2\r\n$1\r\n1\r\n$4\r\n2.14\r\n");

        assert_eq!(to_string(&(Vec::new() as Vec<f64>)).unwrap(), "*0\r\n");
        assert_eq!(to_string(&vec![1 as f64, 2.14 as f64]).unwrap(), "*2\r\n$1\r\n1\r\n$4\r\n2.14\r\n");

        // char
        assert_eq!(to_string(&(Vec::new() as Vec<char>)).unwrap(), "*0\r\n");
        assert_eq!(to_string(&vec!['a', 'b']).unwrap(), "*2\r\n$1\r\na\r\n$1\r\nb\r\n");

        // Vec<Vec<T>>
        assert_eq!(to_string(&(Vec::new() as Vec<Vec<char>>)).unwrap(), "*0\r\n");
        assert_eq!(to_string(&vec![vec!['a'], vec!['b', 'c']]).unwrap(), "*2\r\n*1\r\n$1\r\na\r\n*2\r\n$1\r\nb\r\n$1\r\nc\r\n");
    }

    #[test]
    fn test_serialize_tuple() {
        assert_eq!(to_string(&("mykey", 10)).unwrap(), "*2\r\n$5\r\nmykey\r\n$2\r\n10\r\n");
        assert_eq!(to_string(&("mykey", vec!['a', 'b'])).unwrap(), "*2\r\n$5\r\nmykey\r\n*2\r\n$1\r\na\r\n$1\r\nb\r\n");
        assert_eq!(to_string(&("mykey", (10, 'a'))).unwrap(), "*2\r\n$5\r\nmykey\r\n*2\r\n$2\r\n10\r\n$1\r\na\r\n");
    }

    #[test]
    fn test_serialize_tuple_struct() {
        #[derive(Serialize)]
        struct Tuple<'a, T>(
            &'a str,
            T
        );

        assert_eq!(to_string(&Tuple("mykey", 10)).unwrap(), "*2\r\n$5\r\nmykey\r\n$2\r\n10\r\n");
        assert_eq!(to_string(&Tuple("mykey", vec!['a', 'b'])).unwrap(), "*2\r\n$5\r\nmykey\r\n*2\r\n$1\r\na\r\n$1\r\nb\r\n");
        assert_eq!(to_string(&Tuple("mykey", (10, 'a'))).unwrap(), "*2\r\n$5\r\nmykey\r\n*2\r\n$2\r\n10\r\n$1\r\na\r\n");
    }

    #[test]
    fn test_serialize_num_types() {
        let ser = Serializer { output: "".to_owned() };
        assert_eq!(ser.serialize_num_types(100 as i8), "$3\r\n100\r\n");
        assert_eq!(ser.serialize_num_types(100 as i16), "$3\r\n100\r\n");
        assert_eq!(ser.serialize_num_types(100 as i32), "$3\r\n100\r\n");
        assert_eq!(ser.serialize_num_types(100 as i64), "$3\r\n100\r\n");

        assert_eq!(ser.serialize_num_types(100 as u8), "$3\r\n100\r\n");
        assert_eq!(ser.serialize_num_types(100 as u16), "$3\r\n100\r\n");
        assert_eq!(ser.serialize_num_types(100 as u32), "$3\r\n100\r\n");
        assert_eq!(ser.serialize_num_types(100 as u64), "$3\r\n100\r\n");

        assert_eq!(ser.serialize_num_types(3.14 as f32), "$4\r\n3.14\r\n");
        assert_eq!(ser.serialize_num_types(3.14 as f64), "$4\r\n3.14\r\n");
    }

    #[test]
    fn test_serialize_null() {
        let ser = Serializer { output: "".to_owned() };
        assert_eq!(ser.serialize_null(), "$-1\r\n");
    }

    #[test]
    fn test_serialize_unit() {
        assert_eq!(to_string(&()).unwrap(), "$-1\r\n");
    }

    #[test]
    fn test_serialize_none() {
        assert_eq!(to_string(&(None as Option<bool>)).unwrap(), "$-1\r\n");

        assert_eq!(to_string(&(None as Option<char>)).unwrap(), "$-1\r\n");

        assert_eq!(to_string(&(None as Option<i8>)).unwrap(), "$-1\r\n");
        assert_eq!(to_string(&(None as Option<i16>)).unwrap(), "$-1\r\n");
        assert_eq!(to_string(&(None as Option<i32>)).unwrap(), "$-1\r\n");
        assert_eq!(to_string(&(None as Option<i64>)).unwrap(), "$-1\r\n");

        assert_eq!(to_string(&(None as Option<u8>)).unwrap(), "$-1\r\n");
        assert_eq!(to_string(&(None as Option<u16>)).unwrap(), "$-1\r\n");
        assert_eq!(to_string(&(None as Option<u32>)).unwrap(), "$-1\r\n");
        assert_eq!(to_string(&(None as Option<u64>)).unwrap(), "$-1\r\n");

        assert_eq!(to_string(&(None as Option<f32>)).unwrap(), "$-1\r\n");
        assert_eq!(to_string(&(None as Option<f64>)).unwrap(), "$-1\r\n");

        assert_eq!(to_string(&(None as Option<String>)).unwrap(), "$-1\r\n");
    }

    #[test]
    fn test_serialize_some() {
        assert_eq!(to_string(&Some(true)).unwrap(), "$4\r\ntrue\r\n");

        assert_eq!(to_string(&Some(-1 as i8)).unwrap(), "$2\r\n-1\r\n");
        assert_eq!(to_string(&Some(-1 as i16)).unwrap(), "$2\r\n-1\r\n");
        assert_eq!(to_string(&Some(-100 as i32)).unwrap(), "$4\r\n-100\r\n");
        assert_eq!(to_string(&Some(-100 as i64)).unwrap(), "$4\r\n-100\r\n");

        assert_eq!(to_string(&Some(1 as u8)).unwrap(), "$1\r\n1\r\n");
        assert_eq!(to_string(&Some(1 as u16)).unwrap(), "$1\r\n1\r\n");
        assert_eq!(to_string(&Some(100 as u32)).unwrap(), "$3\r\n100\r\n");
        assert_eq!(to_string(&Some(100 as u64)).unwrap(), "$3\r\n100\r\n");

        assert_eq!(to_string(&Some(-1.34 as f32)).unwrap(), "$5\r\n-1.34\r\n");
        assert_eq!(to_string(&Some(-1.34 as f64)).unwrap(), "$5\r\n-1.34\r\n");

        assert_eq!(to_string(&Some('a')).unwrap(), "$1\r\na\r\n");

        assert_eq!(to_string(&Some("foobar")).unwrap(), "$6\r\nfoobar\r\n");

        assert_eq!(to_string(&Some(vec![vec!['a'], vec!['b', 'c']])).unwrap(), "*2\r\n*1\r\n$1\r\na\r\n*2\r\n$1\r\nb\r\n$1\r\nc\r\n");

        assert_eq!(to_string(&Some(("mykey", (10, 'a')))).unwrap(), "*2\r\n$5\r\nmykey\r\n*2\r\n$2\r\n10\r\n$1\r\na\r\n");

        #[derive(Serialize)]
        struct Tuple<'a, T>(
            &'a str,
            T
        );
        assert_eq!(to_string(&Some(Tuple("mykey", (10, 'a')))).unwrap(), "*2\r\n$5\r\nmykey\r\n*2\r\n$2\r\n10\r\n$1\r\na\r\n");
    }
}
