extern crate serde_resp;

use serde_resp::{serialize, deserialize};
type MyData = (i32, u32, String, bool);

fn main() {
    let mydata: MyData = (-12345, 98765, String::from("Hello world"), true);
    let serialized = serialize(&mydata).unwrap();
    assert_eq!(&serialized, "*4\r\n:-12345\r\n:98765\r\n$11\r\nHello world\r\n+true\r\n");
    let deserialized: MyData = deserialize(&serialized).unwrap();
    assert_eq!(deserialized, mydata);
}