use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;

use magnus::{exception, Error, RArray, RHash, Symbol, TryConvert, Value};
use tk::tokenizer::{
    Model, PaddingDirection, PaddingParams, PaddingStrategy,
    TruncationDirection, TruncationParams, TruncationStrategy, TokenizerImpl
};
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

struct RbArrayStr(Vec<String>);

impl TryConvert for RbArrayStr {
    fn try_convert(ob: Value) -> RbResult<Self> {
        let seq = ob.try_convert::<Vec<String>>()?;
        Ok(Self(seq))
    }
}

impl From<RbArrayStr> for tk::InputSequence<'_> {
    fn from(s: RbArrayStr) -> Self {
        s.0.into()
    }
}

struct PreTokenizedInputSequence<'s>(tk::InputSequence<'s>);

impl<'s> TryConvert for PreTokenizedInputSequence<'s> {
    fn try_convert(ob: Value) -> RbResult<Self> {
        if let Ok(seq) = ob.try_convert::<RbArrayStr>() {
            return Ok(Self(seq.into()));
        }
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
        // TODO check if this branch is needed
        if let Ok(arr) = ob.try_convert::<RArray>() {
            if arr.len() == 2 {
                let first = arr.entry::<TextInputSequence>(0).unwrap();
                let second = arr.entry::<TextInputSequence>(1).unwrap();
                return Ok(Self((first, second).into()));
            }
        }
        Err(Error::new(
            exception::type_error(),
            "TextEncodeInput must be a string or pair of strings",
        ))
    }
}

impl<'s> From<TextEncodeInput<'s>> for tk::tokenizer::EncodeInput<'s> {
    fn from(i: TextEncodeInput<'s>) -> Self {
        i.0
    }
}

struct PreTokenizedEncodeInput<'s>(tk::EncodeInput<'s>);

impl<'s> TryConvert for PreTokenizedEncodeInput<'s> {
    fn try_convert(ob: Value) -> RbResult<Self> {
        if let Ok(i) = ob.try_convert::<PreTokenizedInputSequence>() {
            return Ok(Self(i.into()));
        }
        if let Ok((i1, i2)) =
            ob.try_convert::<(PreTokenizedInputSequence, PreTokenizedInputSequence)>()
        {
            return Ok(Self((i1, i2).into()));
        }
        // TODO check if this branch is needed
        if let Ok(arr) = ob.try_convert::<RArray>() {
            if arr.len() == 2 {
                let first = arr.entry::<PreTokenizedInputSequence>(0).unwrap();
                let second = arr.entry::<PreTokenizedInputSequence>(1).unwrap();
                return Ok(Self((first, second).into()));
            }
        }
        Err(Error::new(
            exception::type_error(),
            "PreTokenizedEncodeInput must be an array of strings or pair of arrays",
        ))
    }
}

impl<'s> From<PreTokenizedEncodeInput<'s>> for tk::tokenizer::EncodeInput<'s> {
    fn from(i: PreTokenizedEncodeInput<'s>) -> Self {
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

    pub fn to_str(&self, pretty: bool) -> RbResult<String> {
        self.tokenizer.borrow().to_string(pretty).map_err(RbError::from)
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

    pub fn save(&self, path: String, pretty: bool) -> RbResult<()> {
        self.tokenizer
            .borrow()
            .save(&path, pretty)
            .map_err(RbError::from)
    }

    pub fn add_tokens(&self, tokens: Vec<String>) -> usize {
        let tokens: Vec<AddedToken> = tokens.iter().map(|t| AddedToken::from(t, true)).collect();
        self.tokenizer.borrow_mut().add_tokens(&tokens)
    }

    pub fn encode(
        &self,
        sequence: Value,
        pair: Option<Value>,
        is_pretokenized: bool,
        add_special_tokens: bool,
    ) -> RbResult<RbEncoding> {
        let sequence: tk::InputSequence = if is_pretokenized {
            sequence.try_convert::<PreTokenizedInputSequence>()?.into()
        } else {
            sequence.try_convert::<TextInputSequence>()?.into()
        };
        let input = match pair {
            Some(pair) => {
                let pair: tk::InputSequence = if is_pretokenized {
                    pair.try_convert::<PreTokenizedInputSequence>()?.into()
                } else {
                    pair.try_convert::<TextInputSequence>()?.into()
                };
                tk::EncodeInput::Dual(sequence, pair)
            }
            None => tk::EncodeInput::Single(sequence),
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
                    o?.try_convert::<PreTokenizedEncodeInput>()?.into()
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
                    .map(Into::<RbEncoding>::into)
                    .collect()
            })
            .map_err(RbError::from)
    }

    pub fn decode(&self, ids: Vec<u32>, skip_special_tokens: bool) -> RbResult<String> {
        self.tokenizer
            .borrow()
            .decode(ids, skip_special_tokens)
            .map_err(RbError::from)
    }

    pub fn decode_batch(&self, sequences: Vec<Vec<u32>>, skip_special_tokens: bool) -> RbResult<Vec<String>> {
        self.tokenizer
            .borrow()
            .decode_batch(sequences, skip_special_tokens)
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

        let value: Value = kwargs.delete(Symbol::new("direction"))?;
        if !value.is_nil() {
            let dir_str: String = value.try_convert()?;
            params.direction = match dir_str.as_str() {
                "left" => PaddingDirection::Left,
                "right" => PaddingDirection::Right,
                _ => return Err(Error::new(exception::arg_error(), "The direction value must be 'left' or 'right'")),
            }
        }

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

        let value: Value = kwargs.delete(Symbol::new("pad_to_multiple_of"))?;
        if !value.is_nil() {
            params.pad_to_multiple_of = value.try_convert()?;
        }

        let value: Value = kwargs.delete(Symbol::new("length"))?;
        if value.is_nil() {
            params.strategy = PaddingStrategy::BatchLongest;
        } else {
            params.strategy = PaddingStrategy::Fixed(value.try_convert()?);
        }

        if !kwargs.is_empty() {
            // TODO improve message
            return Err(Error::new(exception::arg_error(), "unknown keyword"));
        }

        self.tokenizer.borrow_mut().with_padding(Some(params));

        Ok(())
    }

    pub fn no_padding(&self) {
        self.tokenizer.borrow_mut().with_padding(None);
    }

    pub fn padding(&self) -> RbResult<Option<RHash>> {
        self.tokenizer.borrow().get_padding().map_or(Ok(None), |params| {
            let ret_hash = RHash::new();

            ret_hash.aset(
                "length",
                match params.strategy {
                    tk::PaddingStrategy::BatchLongest => None,
                    tk::PaddingStrategy::Fixed(size) => Some(size),
                },
            )?;
            ret_hash.aset("pad_id", params.pad_id)?;
            ret_hash.aset("pad_type_id", params.pad_type_id)?;
            ret_hash.aset("pad_token", &*params.pad_token)?;
            ret_hash.aset("pad_to_multiple_of", params.pad_to_multiple_of)?;

            ret_hash.aset("direction", params.direction.as_ref())?;

            Ok(Some(ret_hash))
        })
    }

    pub fn enable_truncation(&self, max_length: usize, kwargs: RHash) -> RbResult<()> {
        let mut params = TruncationParams {
            max_length,
            ..Default::default()
        };

        let value: Value = kwargs.delete(Symbol::new("stride"))?;
        if !value.is_nil() {
            params.stride = value.try_convert()?;
        }

        let value: Value = kwargs.delete(Symbol::new("direction"))?;
        if !value.is_nil() {
            let dir_str: String = value.try_convert()?;
            params.direction = match dir_str.as_str() {
                "left" => TruncationDirection::Left,
                "right" => TruncationDirection::Right,
                _ => return Err(Error::new(exception::arg_error(), "The direction value must be 'left' or 'right'")),
            }
        }

        let value: Value = kwargs.delete(Symbol::new("strategy"))?;
        if !value.is_nil() {
            let strategy_str: String = value.try_convert()?;
            params.strategy = match strategy_str.as_str() {
                "longest_first" => TruncationStrategy::LongestFirst,
                "only_first" => TruncationStrategy::OnlyFirst,
                "only_second" => TruncationStrategy::OnlySecond,
                _ => return Err(Error::new(exception::arg_error(), "The strategy value must be 'longest_first', 'only_first', or 'only_second'")),
            }
        }

        if !kwargs.is_empty() {
            // TODO improve message
            return Err(Error::new(exception::arg_error(), "unknown keyword"));
        }

        self.tokenizer.borrow_mut().with_truncation(Some(params));

        Ok(())
    }


    pub fn no_truncation(&self) {
        self.tokenizer.borrow_mut().with_truncation(None);
    }

    pub fn truncation(&self) -> RbResult<Option<RHash>> {
        self.tokenizer.borrow().get_truncation().map_or(Ok(None), |params| {
            let ret_hash = RHash::new();

            ret_hash.aset("max_length", params.max_length)?;
            ret_hash.aset("stride", params.stride)?;
            ret_hash.aset("strategy", params.strategy.as_ref())?;
            ret_hash.aset("direction", params.direction.as_ref())?;

            Ok(Some(ret_hash))
        })
    }

    pub fn vocab(&self, with_added_tokens: bool) -> HashMap<String, u32> {
        self.tokenizer.borrow().get_vocab(with_added_tokens)
    }

    pub fn vocab_size(&self, with_added_tokens: bool) -> usize {
        self.tokenizer.borrow().get_vocab_size(with_added_tokens)
    }
}
