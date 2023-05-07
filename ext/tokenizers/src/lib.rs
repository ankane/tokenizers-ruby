#![allow(clippy::new_ret_no_self)]

extern crate tokenizers as tk;

mod decoders;
mod encoding;
mod error;
mod models;
mod normalizers;
mod pre_tokenizers;
mod processors;
mod tokenizer;
mod trainers;
mod utils;

use encoding::RbEncoding;
use error::RbError;
use tokenizer::RbTokenizer;
use utils::RbRegex;

use magnus::{define_module, function, memoize, method, prelude::*, Error, RModule};

type RbResult<T> = Result<T, Error>;

fn module() -> RModule {
    *memoize!(RModule: define_module("Tokenizers").unwrap())
}

fn decoders() -> RModule {
    *memoize!(RModule: module().const_get("Decoders").unwrap())
}

fn models() -> RModule {
    *memoize!(RModule: module().const_get("Models").unwrap())
}

fn normalizers() -> RModule {
    *memoize!(RModule: module().const_get("Normalizers").unwrap())
}

fn pre_tokenizers() -> RModule {
    *memoize!(RModule: module().const_get("PreTokenizers").unwrap())
}

fn processors() -> RModule {
    *memoize!(RModule: module().const_get("Processors").unwrap())
}

fn trainers() -> RModule {
    *memoize!(RModule: module().const_get("Trainers").unwrap())
}

#[magnus::init]
fn init() -> RbResult<()> {
    let module = module();

    let class = module.define_class("Tokenizer", Default::default())?;
    class.define_singleton_method("new", function!(RbTokenizer::from_model, 1))?;
    class.define_singleton_method("from_file", function!(RbTokenizer::from_file, 1))?;
    class.define_method(
        "add_special_tokens",
        method!(RbTokenizer::add_special_tokens, 1),
    )?;
    class.define_method("train", method!(RbTokenizer::train, 2))?;
    class.define_method("_save", method!(RbTokenizer::save, 2))?;
    class.define_method("add_tokens", method!(RbTokenizer::add_tokens, 1))?;
    class.define_method("_encode", method!(RbTokenizer::encode, 4))?;
    class.define_method("_encode_batch", method!(RbTokenizer::encode_batch, 3))?;
    class.define_method("_decode", method!(RbTokenizer::decode, 2))?;
    class.define_method("_decode_batch", method!(RbTokenizer::decode_batch, 2))?;
    class.define_method("decoder=", method!(RbTokenizer::set_decoder, 1))?;
    class.define_method("pre_tokenizer=", method!(RbTokenizer::set_pre_tokenizer, 1))?;
    class.define_method(
        "post_processor=",
        method!(RbTokenizer::set_post_processor, 1),
    )?;
    class.define_method("normalizer=", method!(RbTokenizer::set_normalizer, 1))?;
    class.define_method("token_to_id", method!(RbTokenizer::token_to_id, 1))?;
    class.define_method("id_to_token", method!(RbTokenizer::id_to_token, 1))?;
    class.define_method("_enable_padding", method!(RbTokenizer::enable_padding, 1))?;
    class.define_method("padding", method!(RbTokenizer::padding, 0))?;
    class.define_method("no_padding", method!(RbTokenizer::no_padding, 0))?;
    class.define_method("_enable_truncation", method!(RbTokenizer::enable_truncation, 2))?;
    class.define_method("truncation", method!(RbTokenizer::truncation, 0))?;
    class.define_method("no_truncation", method!(RbTokenizer::no_truncation, 0))?;
    class.define_method("num_special_tokens_to_add", method!(RbTokenizer::num_special_tokens_to_add, 1))?;
    class.define_method("_vocab", method!(RbTokenizer::vocab, 1))?;
    class.define_method("_vocab_size", method!(RbTokenizer::vocab_size, 1))?;
    class.define_method("_to_s", method!(RbTokenizer::to_str, 1))?;

    let class = module.define_class("Encoding", Default::default())?;
    class.define_method("n_sequences", method!(RbEncoding::n_sequences, 0))?;
    class.define_method("ids", method!(RbEncoding::ids, 0))?;
    class.define_method("tokens", method!(RbEncoding::tokens, 0))?;
    class.define_method("word_ids", method!(RbEncoding::word_ids, 0))?;
    class.define_method("sequence_ids", method!(RbEncoding::sequence_ids, 0))?;
    class.define_method("type_ids", method!(RbEncoding::type_ids, 0))?;
    class.define_method("offsets", method!(RbEncoding::offsets, 0))?;
    class.define_method(
        "special_tokens_mask",
        method!(RbEncoding::special_tokens_mask, 0),
    )?;
    class.define_method("attention_mask", method!(RbEncoding::attention_mask, 0))?;
    class.define_method("overflowing", method!(RbEncoding::overflowing, 0))?;
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

    let class = module.define_class("Regex", Default::default())?;
    class.define_singleton_method("new", function!(RbRegex::new, 1))?;

    let models = module.define_module("Models")?;
    let pre_tokenizers = module.define_module("PreTokenizers")?;
    let decoders = module.define_module("Decoders")?;
    let processors = module.define_module("Processors")?;
    let normalizers = module.define_module("Normalizers")?;
    let trainers = module.define_module("Trainers")?;

    models::models(&models)?;
    pre_tokenizers::pre_tokenizers(&pre_tokenizers)?;
    decoders::decoders(&decoders)?;
    processors::processors(&processors)?;
    normalizers::normalizers(&normalizers)?;
    trainers::trainers(&trainers)?;

    Ok(())
}
