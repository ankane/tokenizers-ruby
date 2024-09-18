use magnus::{prelude::*, value::Lazy, Error, ExceptionClass, Ruby};

use super::TOKENIZERS;

pub struct RbError {}

impl RbError {
    // convert to Error instead of Self
    pub fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Error {
        Error::new(error(), e.to_string())
    }

    pub fn new_err(s: String) -> Error {
        Error::new(error(), s)
    }
}

static ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| ruby.get_inner(&TOKENIZERS).const_get("Error").unwrap());

fn error() -> ExceptionClass {
    Ruby::get().unwrap().get_inner(&ERROR)
}
