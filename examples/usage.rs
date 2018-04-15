extern crate serde_resp;

use serde_resp::to_string;

fn main() {
    println!("{:?}", to_string(&"foobar").unwrap());
}