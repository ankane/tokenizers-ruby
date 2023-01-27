use std::cell::RefCell;
use std::path::PathBuf;

use magnus::{exception, Error, RArray, RHash, Symbol, TryConvert, Value};
use tk::tokenizer::{Model, PaddingParams, TokenizerImpl};
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

struct TextInputSequence<'s>(tk::InputSequence<'s>);

impl<'s> TryConvert for TextInputSequence<'s> {
    fn try_convert(ob: Value) -> RbResult<Self> {
        Ok(Self(ob.try_convert::<String>()?.into()))
    }
}

impl<'s> From<TextInputSequence<'s>> for tk::InputSequence<'s> {
    fn from(s: TextInputSequence<'s>) -> Self {
        s.0
    }
}

struct PreTokenizedInputSequence<'s>(tk::InputSequence<'s>);

impl<'s> TryConvert for PreTokenizedInputSequence<'s> {
    fn try_convert(_ob: Value) -> RbResult<Self> {
        todo!()
    }
}

impl<'s> From<PreTokenizedInputSequence<'s>> for tk::InputSequence<'s> {
    fn from(s: PreTokenizedInputSequence<'s>) -> Self {
        s.0
    }
}

struct TextEncodeInput<'s>(tk::EncodeInput<'s>);

impl<'s> TryConvert for TextEncodeInput<'s> {
    fn try_convert(ob: Value) -> RbResult<Self> {
        if let Ok(i) = ob.try_convert::<TextInputSequence>() {
            return Ok(Self(i.into()));
        }
        if let Ok((i1, i2)) = ob.try_convert::<(TextInputSequence, TextInputSequence)>() {
            return Ok(Self((i1, i2).into()));
        }
        todo!()
    }
}

impl<'s> From<TextEncodeInput<'s>> for tk::tokenizer::EncodeInput<'s> {
    fn from(i: TextEncodeInput<'s>) -> Self {
        i.0
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

    pub fn add_special_tokens(&self, tokens: Vec<String>) -> usize {
        let tokens: Vec<AddedToken> = tokens.iter().map(|t| AddedToken::from(t, true)).collect();
        self.tokenizer.borrow_mut().add_special_tokens(&tokens)
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

    pub fn add_tokens(&self, tokens: Vec<String>) -> usize {
        let tokens: Vec<AddedToken> = tokens.iter().map(|t| AddedToken::from(t, true)).collect();
        self.tokenizer.borrow_mut().add_tokens(&tokens)
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

    pub fn encode_batch(
        &self,
        input: RArray,
        is_pretokenized: bool,
        add_special_tokens: bool,
    ) -> RbResult<RArray> {
        let input: Vec<tk::EncodeInput> = input
            .each()
            .map(|o| {
                let input: tk::EncodeInput = if is_pretokenized {
                    todo!()
                } else {
                    o?.try_convert::<TextEncodeInput>()?.into()
                };
                Ok(input)
            })
            .collect::<RbResult<Vec<tk::EncodeInput>>>()?;
        self.tokenizer
            .borrow()
            .encode_batch_char_offsets(input, add_special_tokens)
            .map(|encodings| {
                encodings
                    .into_iter()
                    .map(|e| Into::<RbEncoding>::into(e))
                    .collect()
            })
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

    pub fn set_post_processor(&self, processor: &RbPostProcessor) {
        self.tokenizer
            .borrow_mut()
            .with_post_processor(processor.clone());
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

    // TODO support more kwargs
    pub fn enable_padding(&self, kwargs: RHash) -> RbResult<()> {
        let mut params = PaddingParams::default();

        let value: Value = kwargs.delete(Symbol::new("pad_id"))?;
        if !value.is_nil() {
            params.pad_id = value.try_convert()?;
        }

        let value: Value = kwargs.delete(Symbol::new("pad_type_id"))?;
        if !value.is_nil() {
            params.pad_type_id = value.try_convert()?;
        }

        let value: Value = kwargs.delete(Symbol::new("pad_token"))?;
        if !value.is_nil() {
            params.pad_token = value.try_convert()?;
        }

        if !kwargs.is_empty() {
            // TODO improve message
            return Err(Error::new(exception::arg_error(), "unknown keyword"));
        }

        self.tokenizer.borrow_mut().with_padding(Some(params));

        Ok(())
    }
}
