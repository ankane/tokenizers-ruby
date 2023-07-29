use onig::Regex;
use magnus::{exception, prelude::*, value::Lazy, Error, RClass, Ruby};
use crate::{RbResult, TOKENIZERS};

#[magnus::wrap(class = "Tokenizers::Regex")]
pub struct RbRegex {
    pub inner: Regex,
    pub pattern: String,
}

impl RbRegex {
    pub fn new(s: String) -> RbResult<Self> {
        Ok(Self {
            inner: Regex::new(&s).map_err(|e| Error::new(exception::runtime_error(), e.description().to_owned()))?,
            pattern: s,
        })
    }
}

static REGEX: Lazy<RClass> = Lazy::new(|ruby| ruby.get_inner(&TOKENIZERS).const_get("Regex").unwrap());

pub fn regex() -> RClass {
    Ruby::get().unwrap().get_inner(&REGEX)
}
