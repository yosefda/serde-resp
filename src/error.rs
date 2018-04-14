use serde::de::Error as DeError;
use serde::ser::Error as SerError;
use std::fmt::Display;

error_chain! {
    errors {
        UnsupportedOperation(t: String) {
            description("unsupported operation")
            display("unsupported operation: '{}'", t)
        }

        DeError(t: String) {
            description("deserialization error")
            display("deserialization error: '{}'", t)
        }

        SerError(t: String) {
            description("serialization error")
            display("serialization error: '{}'", t)
        }
    }
}

impl DeError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        ErrorKind::DeError(msg.to_string()).into()
    }
}

impl SerError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        ErrorKind::SerError(msg.to_string()).into()
    }
}