#![allow(clippy::new_ret_no_self)]

extern crate tokenizers as tk;

mod decoders;
mod encoding;
mod error;
mod models;
mod normalizers;
mod pre_tokenizers;
mod processors;
mod ruby;
mod tokenizer;
mod trainers;
mod utils;

use error::RbError;
use utils::RbRegex;

use magnus::{function, prelude::*, value::Lazy, Error, RModule, Ruby};

type RbResult<T> = Result<T, Error>;

static TOKENIZERS: Lazy<RModule> =
    Lazy::new(|ruby| ruby.class_object().const_get("Tokenizers").unwrap());

static DECODERS: Lazy<RModule> =
    Lazy::new(|ruby| ruby.get_inner(&TOKENIZERS).const_get("Decoders").unwrap());

static MODELS: Lazy<RModule> =
    Lazy::new(|ruby| ruby.get_inner(&TOKENIZERS).const_get("Models").unwrap());

static NORMALIZERS: Lazy<RModule> = Lazy::new(|ruby| {
    ruby.get_inner(&TOKENIZERS)
        .const_get("Normalizers")
        .unwrap()
});

static PRE_TOKENIZERS: Lazy<RModule> = Lazy::new(|ruby| {
    ruby.get_inner(&TOKENIZERS)
        .const_get("PreTokenizers")
        .unwrap()
});

static PROCESSORS: Lazy<RModule> =
    Lazy::new(|ruby| ruby.get_inner(&TOKENIZERS).const_get("Processors").unwrap());

static TRAINERS: Lazy<RModule> =
    Lazy::new(|ruby| ruby.get_inner(&TOKENIZERS).const_get("Trainers").unwrap());

#[magnus::init(name = "tokenizers")]
fn init(ruby: &Ruby) -> RbResult<()> {
    let module = ruby.define_module("Tokenizers")?;

    let class = module.define_class("Regex", ruby.class_object())?;
    class.define_singleton_method("new", function!(RbRegex::new, 1))?;

    let models = module.define_module("Models")?;
    let pre_tokenizers = module.define_module("PreTokenizers")?;
    let decoders = module.define_module("Decoders")?;
    let processors = module.define_module("Processors")?;
    let normalizers = module.define_module("Normalizers")?;
    let trainers = module.define_module("Trainers")?;

    tokenizer::init_tokenizer(ruby, &module)?;
    encoding::init_encoding(ruby, &module)?;
    models::init_models(ruby, &models)?;
    pre_tokenizers::init_pre_tokenizers(ruby, &pre_tokenizers)?;
    decoders::init_decoders(ruby, &decoders)?;
    processors::init_processors(ruby, &processors)?;
    normalizers::init_normalizers(ruby, &normalizers)?;
    trainers::init_trainers(ruby, &trainers)?;

    Ok(())
}
