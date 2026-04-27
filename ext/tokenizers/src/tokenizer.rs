use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use magnus::prelude::*;
use magnus::{function, method, Error, RArray, RHash, RModule, RString, Ruby, TryConvert, Value};
use tk::tokenizer::{
    Model, PaddingDirection, PaddingParams, PaddingStrategy, TokenizerImpl, TruncationDirection,
    TruncationParams, TruncationStrategy,
};
use tk::AddedToken;

use crate::tk::PostProcessor;

use super::decoders::RbDecoder;
use super::encoding::RbEncoding;
use super::models::RbModel;
use super::normalizers::RbNormalizer;
use super::pre_tokenizers::RbPreTokenizer;
use super::processors::RbPostProcessor;
use super::ruby::GvlExt;
use super::trainers::RbTrainer;
use super::{RbError, RbResult};

#[magnus::wrap(class = "Tokenizers::AddedToken")]
pub struct RbAddedToken {
    pub content: String,
    pub special: bool,
    pub single_word: Option<bool>,
    pub lstrip: Option<bool>,
    pub rstrip: Option<bool>,
    pub normalized: Option<bool>,
}

impl RbAddedToken {
    pub fn from<S: Into<String>>(content: S, special: Option<bool>) -> Self {
        Self {
            content: content.into(),
            special: special.unwrap_or(false),
            single_word: None,
            lstrip: None,
            rstrip: None,
            normalized: None,
        }
    }

    pub fn get_token(&self) -> tk::tokenizer::AddedToken {
        let mut token = tk::AddedToken::from(&self.content, self.special);

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

impl From<tk::AddedToken> for RbAddedToken {
    fn from(token: tk::AddedToken) -> Self {
        Self {
            content: token.content,
            single_word: Some(token.single_word),
            lstrip: Some(token.lstrip),
            rstrip: Some(token.rstrip),
            normalized: Some(token.normalized),
            special: token.special,
        }
    }
}

impl RbAddedToken {
    pub fn new(ruby: &Ruby, content: Option<String>, kwargs: RHash) -> RbResult<Self> {
        let mut token = RbAddedToken::from(content.unwrap_or("".to_string()), None);

        let value: Value = kwargs.delete(ruby.to_symbol("single_word"))?;
        if !value.is_nil() {
            token.single_word = TryConvert::try_convert(value)?;
        }

        let value: Value = kwargs.delete(ruby.to_symbol("lstrip"))?;
        if !value.is_nil() {
            token.lstrip = TryConvert::try_convert(value)?;
        }

        let value: Value = kwargs.delete(ruby.to_symbol("rstrip"))?;
        if !value.is_nil() {
            token.rstrip = TryConvert::try_convert(value)?;
        }

        let value: Value = kwargs.delete(ruby.to_symbol("normalized"))?;
        if !value.is_nil() {
            token.normalized = TryConvert::try_convert(value)?;
        }

        let value: Value = kwargs.delete(ruby.to_symbol("special"))?;
        if !value.is_nil() {
            token.special = TryConvert::try_convert(value)?;
        }

        if !kwargs.is_empty() {
            // TODO improve message
            return Err(Error::new(ruby.exception_arg_error(), "unknown keyword"));
        }

        Ok(token)
    }

    pub fn get_content(&self) -> String {
        self.content.to_string()
    }

    pub fn get_rstrip(&self) -> bool {
        self.get_token().rstrip
    }

    pub fn get_lstrip(&self) -> bool {
        self.get_token().lstrip
    }

    pub fn get_single_word(&self) -> bool {
        self.get_token().single_word
    }

    pub fn get_normalized(&self) -> bool {
        self.get_token().normalized
    }

    pub fn get_special(&self) -> bool {
        self.get_token().special
    }
}

struct TextInputSequence<'s>(tk::InputSequence<'s>);

impl TryConvert for TextInputSequence<'_> {
    fn try_convert(ob: Value) -> RbResult<Self> {
        Ok(Self(String::try_convert(ob)?.into()))
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
        let seq = <Vec<String>>::try_convert(ob)?;
        Ok(Self(seq))
    }
}

impl From<RbArrayStr> for tk::InputSequence<'_> {
    fn from(s: RbArrayStr) -> Self {
        s.0.into()
    }
}

struct PreTokenizedInputSequence<'s>(tk::InputSequence<'s>);

impl TryConvert for PreTokenizedInputSequence<'_> {
    fn try_convert(ob: Value) -> RbResult<Self> {
        if let Ok(seq) = RbArrayStr::try_convert(ob) {
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

impl TryConvert for TextEncodeInput<'_> {
    fn try_convert(ob: Value) -> RbResult<Self> {
        let ruby = Ruby::get_with(ob);

        if let Ok(i) = TextInputSequence::try_convert(ob) {
            return Ok(Self(i.into()));
        }
        if let Ok((i1, i2)) = <(TextInputSequence, TextInputSequence)>::try_convert(ob) {
            return Ok(Self((i1, i2).into()));
        }
        // TODO check if this branch is needed
        if let Ok(arr) = RArray::try_convert(ob) {
            if arr.len() == 2 {
                let first = arr.entry::<TextInputSequence>(0)?;
                let second = arr.entry::<TextInputSequence>(1)?;
                return Ok(Self((first, second).into()));
            }
        }
        Err(Error::new(
            ruby.exception_type_error(),
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

impl TryConvert for PreTokenizedEncodeInput<'_> {
    fn try_convert(ob: Value) -> RbResult<Self> {
        let ruby = Ruby::get_with(ob);

        if let Ok(i) = PreTokenizedInputSequence::try_convert(ob) {
            return Ok(Self(i.into()));
        }
        if let Ok((i1, i2)) =
            <(PreTokenizedInputSequence, PreTokenizedInputSequence)>::try_convert(ob)
        {
            return Ok(Self((i1, i2).into()));
        }
        // TODO check if this branch is needed
        if let Ok(arr) = RArray::try_convert(ob) {
            if arr.len() == 2 {
                let first = arr.entry::<PreTokenizedInputSequence>(0)?;
                let second = arr.entry::<PreTokenizedInputSequence>(1)?;
                return Ok(Self((first, second).into()));
            }
        }
        Err(Error::new(
            ruby.exception_type_error(),
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
    tokenizer: Arc<RwLock<Tokenizer>>,
}

impl Clone for RbTokenizer {
    fn clone(&self) -> Self {
        RbTokenizer {
            tokenizer: Arc::clone(&self.tokenizer),
        }
    }
}

impl RbTokenizer {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self {
            tokenizer: Arc::new(RwLock::new(tokenizer)),
        }
    }

    /// Acquire the inner tokenizer for reading; surfaces lock poisoning as a
    /// `PyException` instead of panicking.
    pub(crate) fn read_inner(&self) -> RbResult<RwLockReadGuard<'_, Tokenizer>> {
        self.tokenizer
            .read()
            .map_err(|_| RbError::new_err("Tokenizer RwLock is poisoned"))
    }

    /// Acquire the inner tokenizer for writing; surfaces lock poisoning as a
    /// `PyException` instead of panicking.
    pub(crate) fn write_inner(&self) -> RbResult<RwLockWriteGuard<'_, Tokenizer>> {
        self.tokenizer
            .write()
            .map_err(|_| RbError::new_err("Tokenizer RwLock is poisoned"))
    }

    pub fn from_model(model: &RbModel) -> Self {
        RbTokenizer::new(TokenizerImpl::new(model.clone()))
    }

    pub fn from_str(json: RString) -> RbResult<Self> {
        let tokenizer = Tokenizer::from_str(unsafe { json.as_str()? }).map_err(RbError::from);
        Ok(Self::new(tokenizer?))
    }

    pub fn from_file(path: PathBuf) -> RbResult<Self> {
        let tokenizer = Tokenizer::from_file(path).map_err(RbError::from);
        Ok(Self::new(tokenizer?))
    }

    pub fn to_str(&self, pretty: bool) -> RbResult<String> {
        self.read_inner()?.to_string(pretty).map_err(RbError::from)
    }

    pub fn save(&self, path: String, pretty: bool) -> RbResult<()> {
        self.read_inner()?
            .save(&path, pretty)
            .map_err(RbError::from)
    }

    pub fn num_special_tokens_to_add(&self, is_pair: bool) -> RbResult<usize> {
        Ok(self
            .read_inner()?
            .get_post_processor()
            .map_or(0, |p| p.added_tokens(is_pair)))
    }

    pub fn get_vocab(&self, with_added_tokens: bool) -> RbResult<HashMap<String, u32>> {
        Ok(self.read_inner()?.get_vocab(with_added_tokens))
    }

    pub fn get_added_tokens_decoder(ruby: &Ruby, rb_self: &Self) -> RbResult<RHash> {
        let sorted_map = ruby.hash_new();

        for (key, value) in rb_self.read_inner()?.get_added_tokens_decoder() {
            sorted_map.aset::<u32, RbAddedToken>(key, value.into())?;
        }

        Ok(sorted_map)
    }

    pub fn get_vocab_size(&self, with_added_tokens: bool) -> RbResult<usize> {
        Ok(self.read_inner()?.get_vocab_size(with_added_tokens))
    }

    pub fn enable_truncation(
        ruby: &Ruby,
        rb_self: &Self,
        max_length: usize,
        kwargs: RHash,
    ) -> RbResult<()> {
        let mut params = TruncationParams {
            max_length,
            ..Default::default()
        };

        let value: Value = kwargs.delete(ruby.to_symbol("stride"))?;
        if !value.is_nil() {
            params.stride = TryConvert::try_convert(value)?;
        }

        let value: Value = kwargs.delete(ruby.to_symbol("strategy"))?;
        if !value.is_nil() {
            let strategy_str = String::try_convert(value)?;
            params.strategy = match strategy_str.as_str() {
                "longest_first" => TruncationStrategy::LongestFirst,
                "only_first" => TruncationStrategy::OnlyFirst,
                "only_second" => TruncationStrategy::OnlySecond,
                _ => return Err(Error::new(
                    ruby.exception_arg_error(),
                    "The strategy value must be 'longest_first', 'only_first', or 'only_second'",
                )),
            }
        }

        let value: Value = kwargs.delete(ruby.to_symbol("direction"))?;
        if !value.is_nil() {
            let dir_str = String::try_convert(value)?;
            params.direction = match dir_str.as_str() {
                "left" => TruncationDirection::Left,
                "right" => TruncationDirection::Right,
                _ => {
                    return Err(Error::new(
                        ruby.exception_arg_error(),
                        "The direction value must be 'left' or 'right'",
                    ))
                }
            }
        }

        if !kwargs.is_empty() {
            // TODO improve message
            return Err(Error::new(ruby.exception_arg_error(), "unknown keyword"));
        }

        if let Err(error_message) = rb_self.write_inner()?.with_truncation(Some(params)) {
            return Err(Error::new(
                ruby.exception_arg_error(),
                error_message.to_string(),
            ));
        }

        Ok(())
    }

    pub fn no_truncation(&self) -> RbResult<()> {
        self.write_inner()?
            .with_truncation(None)
            .expect("Failed to set truncation to `None`! This should never happen");
        Ok(())
    }

    pub fn get_truncation(ruby: &Ruby, rb_self: &Self) -> RbResult<Option<RHash>> {
        rb_self
            .read_inner()?
            .get_truncation()
            .map_or(Ok(None), |params| {
                let ret_hash = ruby.hash_new();

                ret_hash.aset("max_length", params.max_length)?;
                ret_hash.aset("stride", params.stride)?;
                ret_hash.aset("strategy", params.strategy.as_ref())?;
                ret_hash.aset("direction", params.direction.as_ref())?;

                Ok(Some(ret_hash))
            })
    }

    // TODO support more kwargs
    pub fn enable_padding(ruby: &Ruby, rb_self: &Self, kwargs: RHash) -> RbResult<()> {
        let mut params = PaddingParams::default();

        let value: Value = kwargs.delete(ruby.to_symbol("direction"))?;
        if !value.is_nil() {
            let dir_str = String::try_convert(value)?;
            params.direction = match dir_str.as_str() {
                "left" => PaddingDirection::Left,
                "right" => PaddingDirection::Right,
                _ => {
                    return Err(Error::new(
                        ruby.exception_arg_error(),
                        "The direction value must be 'left' or 'right'",
                    ))
                }
            }
        }

        let value: Value = kwargs.delete(ruby.to_symbol("pad_to_multiple_of"))?;
        if !value.is_nil() {
            params.pad_to_multiple_of = TryConvert::try_convert(value)?;
        }

        let value: Value = kwargs.delete(ruby.to_symbol("pad_id"))?;
        if !value.is_nil() {
            params.pad_id = TryConvert::try_convert(value)?;
        }

        let value: Value = kwargs.delete(ruby.to_symbol("pad_type_id"))?;
        if !value.is_nil() {
            params.pad_type_id = TryConvert::try_convert(value)?;
        }

        let value: Value = kwargs.delete(ruby.to_symbol("pad_token"))?;
        if !value.is_nil() {
            params.pad_token = TryConvert::try_convert(value)?;
        }

        let value: Value = kwargs.delete(ruby.to_symbol("length"))?;
        if value.is_nil() {
            params.strategy = PaddingStrategy::BatchLongest;
        } else {
            params.strategy = PaddingStrategy::Fixed(TryConvert::try_convert(value)?);
        }

        if !kwargs.is_empty() {
            // TODO improve message
            return Err(Error::new(ruby.exception_arg_error(), "unknown keyword"));
        }

        rb_self.write_inner()?.with_padding(Some(params));

        Ok(())
    }

    pub fn no_padding(&self) -> RbResult<()> {
        self.write_inner()?.with_padding(None);
        Ok(())
    }

    pub fn get_padding(ruby: &Ruby, rb_self: &Self) -> RbResult<Option<RHash>> {
        rb_self
            .read_inner()?
            .get_padding()
            .map_or(Ok(None), |params| {
                let ret_hash = ruby.hash_new();

                ret_hash.aset(
                    "length",
                    match params.strategy {
                        tk::PaddingStrategy::BatchLongest => None,
                        tk::PaddingStrategy::Fixed(size) => Some(size),
                    },
                )?;
                ret_hash.aset("pad_to_multiple_of", params.pad_to_multiple_of)?;
                ret_hash.aset("pad_id", params.pad_id)?;
                ret_hash.aset("pad_token", &*params.pad_token)?;
                ret_hash.aset("pad_type_id", params.pad_type_id)?;
                ret_hash.aset("direction", params.direction.as_ref())?;

                Ok(Some(ret_hash))
            })
    }

    pub fn encode(
        &self,
        sequence: Value,
        pair: Option<Value>,
        is_pretokenized: bool,
        add_special_tokens: bool,
    ) -> RbResult<RbEncoding> {
        let sequence: tk::InputSequence = if is_pretokenized {
            PreTokenizedInputSequence::try_convert(sequence)?.into()
        } else {
            TextInputSequence::try_convert(sequence)?.into()
        };
        let input = match pair {
            Some(pair) => {
                let pair: tk::InputSequence = if is_pretokenized {
                    PreTokenizedInputSequence::try_convert(pair)?.into()
                } else {
                    TextInputSequence::try_convert(pair)?.into()
                };
                tk::EncodeInput::Dual(sequence, pair)
            }
            None => tk::EncodeInput::Single(sequence),
        };

        self.read_inner()?
            .encode_char_offsets(input, add_special_tokens)
            .map(|v| RbEncoding { encoding: v })
            .map_err(RbError::from)
    }

    pub fn encode_batch(
        ruby: &Ruby,
        rb_self: &Self,
        input: RArray,
        is_pretokenized: bool,
        add_special_tokens: bool,
    ) -> RbResult<RArray> {
        let input: Vec<tk::EncodeInput> = input
            .into_iter()
            .map(|o| {
                let input: tk::EncodeInput = if is_pretokenized {
                    PreTokenizedEncodeInput::try_convert(o)?.into()
                } else {
                    TextEncodeInput::try_convert(o)?.into()
                };
                Ok(input)
            })
            .collect::<RbResult<Vec<tk::EncodeInput>>>()?;
        ruby.detach(|| {
            rb_self
                .tokenizer
                .read()
                .unwrap()
                .encode_batch_char_offsets(input, add_special_tokens)
        })
        .map(|encodings| ruby.ary_from_iter(encodings.into_iter().map(Into::<RbEncoding>::into)))
        .map_err(RbError::from)
    }

    pub fn encode_batch_fast(
        ruby: &Ruby,
        rb_self: &Self,
        input: RArray,
        is_pretokenized: bool,
        add_special_tokens: bool,
    ) -> RbResult<RArray> {
        let mut items = Vec::<tk::EncodeInput>::with_capacity(input.len());
        for item in input {
            let item: tk::EncodeInput = if is_pretokenized {
                PreTokenizedEncodeInput::try_convert(item)?.into()
            } else {
                TextEncodeInput::try_convert(item)?.into()
            };
            items.push(item);
        }
        ruby.detach(|| {
            rb_self
                .tokenizer
                .read()
                .unwrap()
                .encode_batch_fast(items, add_special_tokens)
        })
        .map(|encodings| ruby.ary_from_iter(encodings.into_iter().map(Into::<RbEncoding>::into)))
        .map_err(RbError::from)
    }

    pub fn decode(&self, ids: Vec<u32>, skip_special_tokens: bool) -> RbResult<String> {
        self.read_inner()?
            .decode(&ids, skip_special_tokens)
            .map_err(RbError::from)
    }

    pub fn decode_batch(
        ruby: &Ruby,
        rb_self: &Self,
        sequences: Vec<Vec<u32>>,
        skip_special_tokens: bool,
    ) -> RbResult<Vec<String>> {
        ruby.detach(|| {
            let slices = sequences.iter().map(|v| &v[..]).collect::<Vec<&[u32]>>();
            rb_self
                .tokenizer
                .read()
                .unwrap()
                .decode_batch(&slices, skip_special_tokens)
        })
        .map_err(RbError::from)
    }

    pub fn token_to_id(&self, token: String) -> RbResult<Option<u32>> {
        Ok(self.read_inner()?.token_to_id(&token))
    }

    pub fn id_to_token(&self, id: u32) -> RbResult<Option<String>> {
        Ok(self.read_inner()?.id_to_token(id))
    }

    pub fn set_encode_special_tokens(&self, value: bool) -> RbResult<()> {
        self.write_inner()?.set_encode_special_tokens(value);
        Ok(())
    }

    pub fn get_encode_special_tokens(&self) -> RbResult<bool> {
        Ok(self.read_inner()?.get_encode_special_tokens())
    }

    pub fn add_tokens(&self, tokens: Vec<String>) -> RbResult<usize> {
        let tokens: Vec<AddedToken> = tokens.iter().map(|t| AddedToken::from(t, true)).collect();
        self.write_inner()?
            .add_tokens(tokens)
            .map_err(RbError::from)
    }

    pub fn add_special_tokens(&self, tokens: Vec<String>) -> RbResult<usize> {
        let tokens: Vec<AddedToken> = tokens.iter().map(|t| AddedToken::from(t, true)).collect();
        self.write_inner()?
            .add_special_tokens(tokens)
            .map_err(RbError::from)
    }

    pub fn train(&self, files: Vec<String>, trainer: Option<&RbTrainer>) -> RbResult<()> {
        let mut trainer = match trainer {
            Some(t) => t.clone(),
            None => self.read_inner()?.get_model().get_trainer(),
        };
        self.write_inner()?
            .train_from_files(&mut trainer, files)
            .map(|_| {})
            .map_err(RbError::from)
    }

    pub fn get_model(&self) -> RbResult<RbModel> {
        Ok(self.read_inner()?.get_model().clone())
    }

    pub fn set_model(&self, model: &RbModel) -> RbResult<()> {
        self.write_inner()?.with_model(model.clone());
        Ok(())
    }

    pub fn get_normalizer(&self) -> RbResult<Option<RbNormalizer>> {
        Ok(self.read_inner()?.get_normalizer().cloned())
    }

    pub fn set_normalizer(&self, normalizer: Option<&RbNormalizer>) -> RbResult<()> {
        self.write_inner()?
            .with_normalizer(normalizer.cloned())
            .map(|_| ())
            .map_err(RbError::from)
    }

    pub fn get_pre_tokenizer(&self) -> RbResult<Option<RbPreTokenizer>> {
        Ok(self.read_inner()?.get_pre_tokenizer().cloned())
    }

    pub fn set_pre_tokenizer(&self, pretok: Option<&RbPreTokenizer>) -> RbResult<()> {
        self.write_inner()?.with_pre_tokenizer(pretok.cloned());
        Ok(())
    }

    pub fn get_post_processor(&self) -> RbResult<Option<RbPostProcessor>> {
        Ok(self.read_inner()?.get_post_processor().cloned())
    }

    pub fn set_post_processor(&self, processor: Option<&RbPostProcessor>) -> RbResult<()> {
        self.write_inner()?.with_post_processor(processor.cloned());
        Ok(())
    }

    pub fn get_decoder(&self) -> RbResult<Option<RbDecoder>> {
        Ok(self.read_inner()?.get_decoder().cloned())
    }

    pub fn set_decoder(&self, decoder: Option<&RbDecoder>) -> RbResult<()> {
        self.write_inner()?.with_decoder(decoder.cloned());
        Ok(())
    }
}

pub fn init_tokenizer(ruby: &Ruby, module: &RModule) -> RbResult<()> {
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
    class.define_method(
        "_encode_batch_fast",
        method!(RbTokenizer::encode_batch_fast, 3),
    )?;
    class.define_method("_decode", method!(RbTokenizer::decode, 2))?;
    class.define_method("_decode_batch", method!(RbTokenizer::decode_batch, 2))?;
    class.define_method(
        "encode_special_tokens",
        method!(RbTokenizer::get_encode_special_tokens, 0),
    )?;
    class.define_method(
        "encode_special_tokens=",
        method!(RbTokenizer::set_encode_special_tokens, 1),
    )?;
    class.define_method("model", method!(RbTokenizer::get_model, 0))?;
    class.define_method("model=", method!(RbTokenizer::set_model, 1))?;
    class.define_method("decoder", method!(RbTokenizer::get_decoder, 0))?;
    class.define_method("decoder=", method!(RbTokenizer::set_decoder, 1))?;
    class.define_method("pre_tokenizer", method!(RbTokenizer::get_pre_tokenizer, 0))?;
    class.define_method("pre_tokenizer=", method!(RbTokenizer::set_pre_tokenizer, 1))?;
    class.define_method(
        "post_processor",
        method!(RbTokenizer::get_post_processor, 0),
    )?;
    class.define_method(
        "post_processor=",
        method!(RbTokenizer::set_post_processor, 1),
    )?;
    class.define_method("normalizer", method!(RbTokenizer::get_normalizer, 0))?;
    class.define_method("normalizer=", method!(RbTokenizer::set_normalizer, 1))?;
    class.define_method("token_to_id", method!(RbTokenizer::token_to_id, 1))?;
    class.define_method("id_to_token", method!(RbTokenizer::id_to_token, 1))?;
    class.define_method("_enable_padding", method!(RbTokenizer::enable_padding, 1))?;
    class.define_method("padding", method!(RbTokenizer::get_padding, 0))?;
    class.define_method("no_padding", method!(RbTokenizer::no_padding, 0))?;
    class.define_method(
        "_enable_truncation",
        method!(RbTokenizer::enable_truncation, 2),
    )?;
    class.define_method("truncation", method!(RbTokenizer::get_truncation, 0))?;
    class.define_method("no_truncation", method!(RbTokenizer::no_truncation, 0))?;
    class.define_method(
        "num_special_tokens_to_add",
        method!(RbTokenizer::num_special_tokens_to_add, 1),
    )?;
    class.define_method("_vocab", method!(RbTokenizer::get_vocab, 1))?;
    class.define_method("_vocab_size", method!(RbTokenizer::get_vocab_size, 1))?;
    class.define_method(
        "added_tokens_decoder",
        method!(RbTokenizer::get_added_tokens_decoder, 0),
    )?;
    class.define_method("_to_s", method!(RbTokenizer::to_str, 1))?;

    let class = module.define_class("AddedToken", ruby.class_object())?;
    class.define_singleton_method("_new", function!(RbAddedToken::new, 2))?;
    class.define_method("content", method!(RbAddedToken::get_content, 0))?;
    class.define_method("rstrip", method!(RbAddedToken::get_rstrip, 0))?;
    class.define_method("lstrip", method!(RbAddedToken::get_lstrip, 0))?;
    class.define_method("single_word", method!(RbAddedToken::get_single_word, 0))?;
    class.define_method("normalized", method!(RbAddedToken::get_normalized, 0))?;
    class.define_method("special", method!(RbAddedToken::get_special, 0))?;

    Ok(())
}
