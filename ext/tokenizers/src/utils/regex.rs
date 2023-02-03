use onig::Regex;
use magnus::{exception, memoize, Error, Module, RClass};
use crate::{module, RbResult};

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

pub fn regex() -> RClass {
    *memoize!(RClass: module().const_get("Regex").unwrap())
}
