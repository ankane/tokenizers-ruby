use std::cell::RefCell;
use std::path::PathBuf;
use tk::tokenizer::Tokenizer;
use tk::AddedToken;

use super::decoders::RbBPEDecoder;
use super::encoding::RbEncoding;
use super::models::RbBPE;
use super::normalizers::RbBertNormalizer;
use super::pre_tokenizers::RbBertPreTokenizer;
use super::{RbError, RbResult};

#[magnus::wrap(class = "Tokenizers::Tokenizer")]
pub struct RbTokenizer {
    tokenizer: RefCell<Tokenizer>,
}

impl RbTokenizer {
    pub fn new(model: &RbBPE) -> Self {
        Self {
            tokenizer: RefCell::new(Tokenizer::new(model.model.clone())),
        }
    }

    pub fn from_file(path: PathBuf) -> RbResult<Self> {
        Tokenizer::from_file(path)
            .map(|v| RbTokenizer {
                tokenizer: RefCell::new(v),
            })
            .map_err(RbError::from)
    }

    pub fn add_special_tokens(&self, tokens: Vec<String>) {
        let tokens: Vec<AddedToken> = tokens.iter().map(|t| AddedToken::from(t, true)).collect();
        self.tokenizer.borrow_mut().add_special_tokens(&tokens);
        // TODO return self
    }

    pub fn encode(&self, sequence: String, add_special_tokens: bool) -> RbResult<RbEncoding> {
        self.tokenizer
            .borrow()
            .encode(sequence, add_special_tokens)
            .map(|v| RbEncoding { encoding: v })
            .map_err(RbError::from)
    }

    pub fn decode(&self, ids: Vec<u32>) -> RbResult<String> {
        self.tokenizer
            .borrow()
            .decode(ids, true)
            .map_err(RbError::from)
    }

    pub fn set_decoder(&self, decoder: &RbBPEDecoder) {
        self.tokenizer
            .borrow_mut()
            .with_decoder(decoder.decoder.clone());
    }

    pub fn set_pre_tokenizer(&self, pre_tokenizer: &RbBertPreTokenizer) {
        self.tokenizer
            .borrow_mut()
            .with_pre_tokenizer(pre_tokenizer.pretok);
    }

    pub fn set_normalizer(&self, normalizer: &RbBertNormalizer) {
        self.tokenizer
            .borrow_mut()
            .with_normalizer(normalizer.normalizer);
    }
}
