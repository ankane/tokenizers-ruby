extern crate tokenizers as tk;

mod decoders;
mod encoding;
mod error;
mod models;
mod normalizers;
mod pre_tokenizers;
mod tokenizer;

use decoders::RbBPEDecoder;
use encoding::RbEncoding;
use error::RbError;
use models::RbBPE;
use normalizers::RbBertNormalizer;
use pre_tokenizers::RbBertPreTokenizer;
use tokenizer::RbTokenizer;

use magnus::{define_module, function, memoize, method, prelude::*, Error, RModule};

type RbResult<T> = Result<T, Error>;

fn module() -> RModule {
    *memoize!(RModule: define_module("Tokenizers").unwrap())
}

#[magnus::init]
fn init() -> RbResult<()> {
    let module = module();
    module.define_singleton_method(
        "_from_pretrained",
        function!(RbTokenizer::from_pretrained, 3),
    )?;

    let class = module.define_class("BPE", Default::default())?;
    class.define_singleton_method("new", function!(RbBPE::new, 2))?;

    let class = module.define_class("Tokenizer", Default::default())?;
    class.define_singleton_method("new", function!(RbTokenizer::new, 1))?;
    class.define_method(
        "add_special_tokens",
        method!(RbTokenizer::add_special_tokens, 1),
    )?;
    class.define_method("encode", method!(RbTokenizer::encode, 1))?;
    class.define_method("decode", method!(RbTokenizer::decode, 1))?;
    class.define_method("decoder=", method!(RbTokenizer::set_decoder, 1))?;
    class.define_method("pre_tokenizer=", method!(RbTokenizer::set_pre_tokenizer, 1))?;
    class.define_method("normalizer=", method!(RbTokenizer::set_normalizer, 1))?;

    let class = module.define_class("Encoding", Default::default())?;
    class.define_method("ids", method!(RbEncoding::ids, 0))?;
    class.define_method("tokens", method!(RbEncoding::tokens, 0))?;

    let class = module.define_class("BPEDecoder", Default::default())?;
    class.define_singleton_method("new", function!(RbBPEDecoder::new, 0))?;

    let class = module.define_class("BertPreTokenizer", Default::default())?;
    class.define_singleton_method("new", function!(RbBertPreTokenizer::new, 0))?;

    let class = module.define_class("BertNormalizer", Default::default())?;
    class.define_singleton_method("new", function!(RbBertNormalizer::new, 0))?;

    Ok(())
}
