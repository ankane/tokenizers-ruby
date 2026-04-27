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

use encoding::RbEncoding;
use error::RbError;
use utils::RbRegex;

use magnus::{function, method, prelude::*, value::Lazy, Error, RModule, Ruby};

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

    let class = module.define_class("Encoding", ruby.class_object())?;
    class.define_method("n_sequences", method!(RbEncoding::get_n_sequences, 0))?;
    class.define_method("ids", method!(RbEncoding::get_ids, 0))?;
    class.define_method("tokens", method!(RbEncoding::get_tokens, 0))?;
    class.define_method("word_ids", method!(RbEncoding::get_word_ids, 0))?;
    class.define_method("sequence_ids", method!(RbEncoding::get_sequence_ids, 0))?;
    class.define_method("type_ids", method!(RbEncoding::get_type_ids, 0))?;
    class.define_method("offsets", method!(RbEncoding::get_offsets, 0))?;
    class.define_method(
        "special_tokens_mask",
        method!(RbEncoding::get_special_tokens_mask, 0),
    )?;
    class.define_method("attention_mask", method!(RbEncoding::get_attention_mask, 0))?;
    class.define_method("overflowing", method!(RbEncoding::get_overflowing, 0))?;
    class.define_method("_word_to_tokens", method!(RbEncoding::word_to_tokens, 2))?;
    class.define_method("_word_to_chars", method!(RbEncoding::word_to_chars, 2))?;
    class.define_method(
        "token_to_sequence",
        method!(RbEncoding::token_to_sequence, 1),
    )?;
    class.define_method("token_to_chars", method!(RbEncoding::token_to_chars, 1))?;
    class.define_method("token_to_word", method!(RbEncoding::token_to_word, 1))?;
    class.define_method("_char_to_token", method!(RbEncoding::char_to_token, 2))?;
    class.define_method("_char_to_word", method!(RbEncoding::char_to_word, 2))?;

    let class = module.define_class("Regex", ruby.class_object())?;
    class.define_singleton_method("new", function!(RbRegex::new, 1))?;

    let models = module.define_module("Models")?;
    let pre_tokenizers = module.define_module("PreTokenizers")?;
    let decoders = module.define_module("Decoders")?;
    let processors = module.define_module("Processors")?;
    let normalizers = module.define_module("Normalizers")?;
    let trainers = module.define_module("Trainers")?;

    tokenizer::init_tokenizer(ruby, &module)?;
    models::init_models(ruby, &models)?;
    pre_tokenizers::init_pre_tokenizers(ruby, &pre_tokenizers)?;
    decoders::init_decoders(ruby, &decoders)?;
    processors::init_processors(ruby, &processors)?;
    normalizers::init_normalizers(ruby, &normalizers)?;
    trainers::init_trainers(ruby, &trainers)?;

    Ok(())
}
