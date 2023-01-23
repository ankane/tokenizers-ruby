use std::cell::RefCell;
use std::path::PathBuf;

use tk::tokenizer::{Model, TokenizerImpl};
use tk::AddedToken;

use super::decoders::RbDecoder;
use super::encoding::RbEncoding;
use super::models::RbModel;
use super::normalizers::RbNormalizer;
use super::pre_tokenizers::RbPreTokenizer;
use super::processors::RbPostProcessor;
use super::trainers::RbTrainer;
use super::{RbError, RbResult};

pub struct RbAddedToken {
    pub content: String,
    pub is_special_token: bool,
    pub single_word: Option<bool>,
    pub lstrip: Option<bool>,
    pub rstrip: Option<bool>,
    pub normalized: Option<bool>,
}

impl RbAddedToken {
    pub fn from<S: Into<String>>(content: S, is_special_token: Option<bool>) -> Self {
        Self {
            content: content.into(),
            is_special_token: is_special_token.unwrap_or(false),
            single_word: None,
            lstrip: None,
            rstrip: None,
            normalized: None,
        }
    }

    pub fn get_token(&self) -> tk::tokenizer::AddedToken {
        let mut token = tk::AddedToken::from(&self.content, self.is_special_token);

        if let Some(sw) = self.single_word {
            token = token.single_word(sw);
        }
        if let Some(ls) = self.lstrip {
            token = token.lstrip(ls);
        }
        if let Some(rs) = self.rstrip {
            token = token.rstrip(rs);
        }
        if let Some(n) = self.normalized {
            token = token.normalized(n);
        }

        token
    }
}

type Tokenizer = TokenizerImpl<RbModel, RbNormalizer, RbPreTokenizer, RbPostProcessor, RbDecoder>;

#[magnus::wrap(class = "Tokenizers::Tokenizer")]
pub struct RbTokenizer {
    tokenizer: RefCell<Tokenizer>,
}

impl RbTokenizer {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self {
            tokenizer: RefCell::new(tokenizer),
        }
    }

    pub fn from_model(model: &RbModel) -> Self {
        RbTokenizer::new(TokenizerImpl::new(model.clone()))
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

    pub fn train(&self, files: Vec<String>, trainer: Option<&RbTrainer>) -> RbResult<()> {
        let mut trainer = trainer.map_or_else(
            || self.tokenizer.borrow().get_model().get_trainer(),
            |t| t.clone(),
        );
        self.tokenizer
            .borrow_mut()
            .train_from_files(&mut trainer, files)
            .map(|_| {})
            .map_err(RbError::from)
    }

    pub fn save(&self, path: String) -> RbResult<()> {
        self.tokenizer
            .borrow()
            .save(&path, false)
            .map_err(RbError::from)
            .into()
    }

    pub fn add_tokens(&self, tokens: Vec<String>) {
        let tokens: Vec<AddedToken> = tokens.iter().map(|t| AddedToken::from(t, true)).collect();
        self.tokenizer.borrow_mut().add_tokens(&tokens);
        // TODO return self
    }

    pub fn encode(
        &self,
        sequence: String,
        pair: Option<String>,
        add_special_tokens: bool,
    ) -> RbResult<RbEncoding> {
        let input = match pair {
            Some(pair) => tk::EncodeInput::Dual(sequence.into(), pair.into()),
            None => tk::EncodeInput::Single(sequence.into()),
        };
        self.tokenizer
            .borrow()
            .encode_char_offsets(input, add_special_tokens)
            .map(|v| RbEncoding { encoding: v })
            .map_err(RbError::from)
    }

    pub fn decode(&self, ids: Vec<u32>) -> RbResult<String> {
        self.tokenizer
            .borrow()
            .decode(ids, true)
            .map_err(RbError::from)
    }

    pub fn set_decoder(&self, decoder: &RbDecoder) {
        self.tokenizer.borrow_mut().with_decoder(decoder.clone());
    }

    pub fn set_pre_tokenizer(&self, pretok: &RbPreTokenizer) {
        self.tokenizer
            .borrow_mut()
            .with_pre_tokenizer(pretok.clone());
    }

    pub fn set_normalizer(&self, normalizer: &RbNormalizer) {
        self.tokenizer
            .borrow_mut()
            .with_normalizer(normalizer.clone());
    }

    pub fn token_to_id(&self, token: String) -> Option<u32> {
        self.tokenizer.borrow().token_to_id(&token)
    }

    pub fn id_to_token(&self, id: u32) -> Option<String> {
        self.tokenizer.borrow().id_to_token(id)
    }
}
