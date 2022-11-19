use magnus::{
    define_module, exception, function, memoize, method, prelude::*, Error, ExceptionClass, RModule,
};
use std::cell::RefCell;
use tokenizers::models::bpe;
use tokenizers::pre_tokenizers::bert;
use tokenizers::{decoders, normalizers, tokenizer, AddedToken};

#[magnus::wrap(class = "Tokenizers::Tokenizer")]
pub struct Tokenizer(RefCell<tokenizer::Tokenizer>);

#[magnus::wrap(class = "Tokenizers::Encoding")]
pub struct Encoding(tokenizers::Encoding);

#[magnus::wrap(class = "Tokenizers::BPE")]
pub struct BPE(bpe::BPE);

#[magnus::wrap(class = "Tokenizers::BPEDecoder")]
pub struct BPEDecoder(decoders::bpe::BPEDecoder);

#[magnus::wrap(class = "Tokenizers::BertPreTokenizer")]
pub struct BertPreTokenizer(bert::BertPreTokenizer);

#[magnus::wrap(class = "Tokenizers::BertNormalizer")]
pub struct BertNormalizer(normalizers::BertNormalizer);

impl Tokenizer {
    pub fn new(model: &BPE) -> Self {
        Self(RefCell::new(tokenizers::Tokenizer::new(model.0.clone())))
    }

    pub fn add_special_tokens(&self, tokens: Vec<String>) {
        let tokens: Vec<AddedToken> = tokens.iter().map(|t| AddedToken::from(t, true)).collect();
        self.0.borrow_mut().add_special_tokens(&tokens);
        // TODO return self
    }

    pub fn encode(&self, text: String) -> Result<Encoding, Error> {
        self.0
            .borrow()
            .encode(text, false)
            .map(Encoding)
            .map_err(|e| Error::new(error(), e.to_string()))
    }

    pub fn decode(&self, ids: Vec<u32>) -> Result<String, Error> {
        self.0
            .borrow()
            .decode(ids, true)
            .map_err(|e| Error::new(error(), e.to_string()))
    }

    pub fn set_decoder(&self, decoder: &BPEDecoder) {
        self.0.borrow_mut().with_decoder(decoder.0.clone());
    }

    pub fn set_pre_tokenizer(&self, pre_tokenizer: &BertPreTokenizer) {
        self.0.borrow_mut().with_pre_tokenizer(pre_tokenizer.0);
    }

    pub fn set_normalizer(&self, normalizer: &BertNormalizer) {
        self.0.borrow_mut().with_normalizer(normalizer.0);
    }
}

impl Encoding {
    pub fn ids(&self) -> Vec<u32> {
        self.0.get_ids().into()
    }

    pub fn tokens(&self) -> Vec<String> {
        self.0.get_tokens().into()
    }
}

impl BPE {
    pub fn new(vocab: String, merges: String) -> Result<Self, Error> {
        bpe::BPE::from_file(&vocab, &merges)
            .unk_token("<unk>".into())
            .end_of_word_suffix("</w>".into())
            .build()
            .map(BPE)
            .map_err(|e| Error::new(error(), e.to_string()))
    }
}

impl BPEDecoder {
    pub fn new() -> Self {
        BPEDecoder(decoders::bpe::BPEDecoder::default())
    }
}

impl BertPreTokenizer {
    pub fn new() -> Self {
        BertPreTokenizer(bert::BertPreTokenizer)
    }
}

impl BertNormalizer {
    pub fn new() -> Self {
        BertNormalizer(normalizers::BertNormalizer::default())
    }
}

fn from_pretrained(
    identifier: String,
    revision: String,
    auth_token: Option<String>,
) -> Result<Tokenizer, Error> {
    let version: String = module().const_get("VERSION").unwrap();
    let params = tokenizers::FromPretrainedParameters {
        revision,
        auth_token,
        user_agent: [("bindings", "Ruby".to_string()), ("version", version)]
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
    };

    tokenizer::Tokenizer::from_pretrained(identifier, Some(params))
        .map(|v| Tokenizer(RefCell::new(v)))
        .map_err(|e| Error::new(error(), e.to_string()))
}

fn module() -> RModule {
    *memoize!(RModule: define_module("Tokenizers").unwrap())
}

fn error() -> ExceptionClass {
    *memoize!(ExceptionClass: module().define_error("Error", exception::standard_error()).unwrap())
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = module();
    module.define_singleton_method("_from_pretrained", function!(from_pretrained, 3))?;

    let class = module.define_class("BPE", Default::default())?;
    class.define_singleton_method("new", function!(BPE::new, 2))?;

    let class = module.define_class("Tokenizer", Default::default())?;
    class.define_singleton_method("new", function!(Tokenizer::new, 1))?;
    class.define_method(
        "add_special_tokens",
        method!(Tokenizer::add_special_tokens, 1),
    )?;
    class.define_method("encode", method!(Tokenizer::encode, 1))?;
    class.define_method("decode", method!(Tokenizer::decode, 1))?;
    class.define_method("decoder=", method!(Tokenizer::set_decoder, 1))?;
    class.define_method("pre_tokenizer=", method!(Tokenizer::set_pre_tokenizer, 1))?;
    class.define_method("normalizer=", method!(Tokenizer::set_normalizer, 1))?;

    let class = module.define_class("Encoding", Default::default())?;
    class.define_method("ids", method!(Encoding::ids, 0))?;
    class.define_method("tokens", method!(Encoding::tokens, 0))?;

    let class = module.define_class("BPEDecoder", Default::default())?;
    class.define_singleton_method("new", function!(BPEDecoder::new, 0))?;

    let class = module.define_class("BertPreTokenizer", Default::default())?;
    class.define_singleton_method("new", function!(BertPreTokenizer::new, 0))?;

    let class = module.define_class("BertNormalizer", Default::default())?;
    class.define_singleton_method("new", function!(BertNormalizer::new, 0))?;

    Ok(())
}
