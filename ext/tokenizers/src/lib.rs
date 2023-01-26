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

use decoders::RbBPEDecoder;
use encoding::RbEncoding;
use error::RbError;
use models::RbBPE;
use normalizers::RbBertNormalizer;
use pre_tokenizers::{RbBertPreTokenizer, RbWhitespace};
use tokenizer::RbTokenizer;
use trainers::RbBpeTrainer;

use magnus::{define_module, function, memoize, method, prelude::*, Error, RModule};

type RbResult<T> = Result<T, Error>;

fn module() -> RModule {
    *memoize!(RModule: define_module("Tokenizers").unwrap())
}

#[magnus::init]
fn init() -> RbResult<()> {
    let module = module();

    let model = module.define_class("Model", Default::default())?;

    let class = module.define_class("BPE", model)?;
    class.define_singleton_method("_new", function!(RbBPE::new, 3))?;
    class.define_singleton_method("_from_file", function!(RbBPE::from_file, 3))?;

    let class = module.define_class("Tokenizer", Default::default())?;
    class.define_singleton_method("new", function!(RbTokenizer::from_model, 1))?;
    class.define_singleton_method("from_file", function!(RbTokenizer::from_file, 1))?;
    class.define_method(
        "add_special_tokens",
        method!(RbTokenizer::add_special_tokens, 1),
    )?;
    class.define_method("train", method!(RbTokenizer::train, 2))?;
    class.define_method("save", method!(RbTokenizer::save, 1))?;
    class.define_method("add_tokens", method!(RbTokenizer::add_tokens, 1))?;
    class.define_method("_encode", method!(RbTokenizer::encode, 3))?;
    class.define_method("decode", method!(RbTokenizer::decode, 1))?;
    class.define_method("decoder=", method!(RbTokenizer::set_decoder, 1))?;
    class.define_method("pre_tokenizer=", method!(RbTokenizer::set_pre_tokenizer, 1))?;
    class.define_method("normalizer=", method!(RbTokenizer::set_normalizer, 1))?;
    class.define_method("token_to_id", method!(RbTokenizer::token_to_id, 1))?;
    class.define_method("id_to_token", method!(RbTokenizer::id_to_token, 1))?;

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

    let decoder = module.define_class("Decoder", Default::default())?;

    let class = module.define_class("BPEDecoder", decoder)?;
    class.define_singleton_method("new", function!(RbBPEDecoder::new, 0))?;

    let normalizer = module.define_class("Normalizer", Default::default())?;

    let class = module.define_class("BertNormalizer", normalizer)?;
    class.define_singleton_method("new", function!(RbBertNormalizer::new, 0))?;

    let trainer = module.define_class("Trainer", Default::default())?;

    let class = module.define_class("BpeTrainer", trainer)?;
    class.define_singleton_method("_new", function!(RbBpeTrainer::new, 1))?;

    let pre_tokenizer = module.define_class("PreTokenizer", Default::default())?;

    let class = module.define_class("BertPreTokenizer", pre_tokenizer)?;
    class.define_singleton_method("new", function!(RbBertPreTokenizer::new, 0))?;

    let class = module.define_class("Whitespace", pre_tokenizer)?;
    class.define_singleton_method("new", function!(RbWhitespace::new, 0))?;

    let _post_processor = module.define_class("PostProcessor", Default::default())?;

    Ok(())
}
