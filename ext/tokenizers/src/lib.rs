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
use tokenizer::{RbAddedToken, RbTokenizer};
use utils::RbRegex;

use magnus::{function, method, prelude::*, value::Lazy, Error, RModule, Ruby};

type RbResult<T> = Result<T, Error>;

static TOKENIZERS: Lazy<RModule> = Lazy::new(|ruby| ruby.class_object().const_get("Tokenizers").unwrap());

static DECODERS: Lazy<RModule> = Lazy::new(|ruby| ruby.get_inner(&TOKENIZERS).const_get("Decoders").unwrap());

static MODELS: Lazy<RModule> = Lazy::new(|ruby| ruby.get_inner(&TOKENIZERS).const_get("Models").unwrap());

static NORMALIZERS: Lazy<RModule> = Lazy::new(|ruby| ruby.get_inner(&TOKENIZERS).const_get("Normalizers").unwrap());

static PRE_TOKENIZERS: Lazy<RModule> = Lazy::new(|ruby| ruby.get_inner(&TOKENIZERS).const_get("PreTokenizers").unwrap());

static PROCESSORS: Lazy<RModule> = Lazy::new(|ruby| ruby.get_inner(&TOKENIZERS).const_get("Processors").unwrap());

static TRAINERS: Lazy<RModule> = Lazy::new(|ruby| ruby.get_inner(&TOKENIZERS).const_get("Trainers").unwrap());

#[magnus::init]
fn init(ruby: &Ruby) -> RbResult<()> {
    let module = ruby.define_module("Tokenizers")?;

    let class = module.define_class("Tokenizer", ruby.class_object())?;
    class.define_singleton_method("new", function!(RbTokenizer::from_model, 1))?;
    class.define_singleton_method("from_str", function!(RbTokenizer::from_str, 1))?;
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
    class.define_method("model", method!(RbTokenizer::get_model, 0))?;
    class.define_method("model=", method!(RbTokenizer::set_model,1))?;
    class.define_method("decoder", method!(RbTokenizer::get_decoder, 0))?;
    class.define_method("decoder=", method!(RbTokenizer::set_decoder, 1))?;
    class.define_method("pre_tokenizer", method!(RbTokenizer::get_pre_tokenizer, 0))?;
    class.define_method("pre_tokenizer=", method!(RbTokenizer::set_pre_tokenizer, 1))?;
    class.define_method("post_processor", method!(RbTokenizer::get_post_processor, 0))?;
    class.define_method(
        "post_processor=",
        method!(RbTokenizer::set_post_processor, 1),
    )?;
    class.define_method("normalizer", method!(RbTokenizer::get_normalizer, 0))?;
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
    class.define_method("added_tokens_decoder", method!(RbTokenizer::get_added_tokens_decoder, 0))?;
    class.define_method("_to_s", method!(RbTokenizer::to_str, 1))?;

    let class = module.define_class("Encoding", ruby.class_object())?;
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

    let class = module.define_class("Regex", ruby.class_object())?;
    class.define_singleton_method("new", function!(RbRegex::new, 1))?;

    let class = module.define_class("AddedToken", ruby.class_object())?;
    class.define_singleton_method("_new", function!(RbAddedToken::new, 2))?;
    class.define_method("content", method!(RbAddedToken::get_content, 0))?;
    class.define_method("rstrip", method!(RbAddedToken::get_rstrip, 0))?;
    class.define_method("lstrip", method!(RbAddedToken::get_lstrip, 0))?;
    class.define_method("single_word", method!(RbAddedToken::get_single_word, 0))?;
    class.define_method("normalized", method!(RbAddedToken::get_normalized, 0))?;
    class.define_method("special", method!(RbAddedToken::get_special, 0))?;

    let models = module.define_module("Models")?;
    let pre_tokenizers = module.define_module("PreTokenizers")?;
    let decoders = module.define_module("Decoders")?;
    let processors = module.define_module("Processors")?;
    let normalizers = module.define_module("Normalizers")?;
    let trainers = module.define_module("Trainers")?;

    models::init_models(ruby, &models)?;
    pre_tokenizers::init_pre_tokenizers(ruby, &pre_tokenizers)?;
    decoders::init_decoders(ruby, &decoders)?;
    processors::init_processors(ruby, &processors)?;
    normalizers::init_normalizers(ruby, &normalizers)?;
    trainers::init_trainers(ruby, &trainers)?;

    Ok(())
}
