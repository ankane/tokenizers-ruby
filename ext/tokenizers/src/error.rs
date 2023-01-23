use magnus::{memoize, Error, ExceptionClass, Module};

use super::module;

pub struct RbError {}

impl RbError {
    // convert to Error instead of Self
    pub fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Error {
        Error::new(error(), e.to_string())
    }
}

fn error() -> ExceptionClass {
    *memoize!(ExceptionClass: module().const_get("Error").unwrap())
}
