use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use crate::trainers::RbTrainer;
use magnus::typed_data::DataTypeBuilder;
use magnus::{
    exception, function, memoize, method, Class, DataType, DataTypeFunctions, Error, Module, Object,
    RClass, RHash, RModule, Symbol, TypedData, Value,
};
use serde::{Deserialize, Serialize};
use tk::models::bpe::{BpeBuilder, Merges, Vocab, BPE};
use tk::models::ModelWrapper;
use tk::models::unigram::Unigram;
use tk::models::wordlevel::WordLevel;
use tk::models::wordpiece::{WordPiece, WordPieceBuilder};
use tk::{Model, Token};

use super::{RbError, RbResult};

#[derive(DataTypeFunctions, Clone, Serialize, Deserialize)]
pub struct RbModel {
    #[serde(flatten)]
    pub model: Arc<RwLock<ModelWrapper>>,
}

impl Model for RbModel {
    type Trainer = RbTrainer;

    fn tokenize(&self, tokens: &str) -> tk::Result<Vec<Token>> {
        self.model.read().unwrap().tokenize(tokens)
    }

    fn token_to_id(&self, token: &str) -> Option<u32> {
        self.model.read().unwrap().token_to_id(token)
    }

    fn id_to_token(&self, id: u32) -> Option<String> {
        self.model.read().unwrap().id_to_token(id)
    }

    fn get_vocab(&self) -> HashMap<String, u32> {
        self.model.read().unwrap().get_vocab()
    }

    fn get_vocab_size(&self) -> usize {
        self.model.read().unwrap().get_vocab_size()
    }

    fn save(&self, folder: &Path, name: Option<&str>) -> tk::Result<Vec<PathBuf>> {
        self.model.read().unwrap().save(folder, name)
    }

    fn get_trainer(&self) -> Self::Trainer {
        self.model.read().unwrap().get_trainer().into()
    }
}

impl<I> From<I> for RbModel
where
    I: Into<ModelWrapper>,
{
    fn from(model: I) -> Self {
        Self {
            model: Arc::new(RwLock::new(model.into())),
        }
    }
}

pub struct RbBPE {}

impl RbBPE {
    fn with_builder(mut builder: BpeBuilder, kwargs: RHash) -> RbResult<RbModel> {
        let value: Value = kwargs.delete(Symbol::new("cache_capacity"))?;
        if !value.is_nil() {
            builder = builder.cache_capacity(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("dropout"))?;
        if !value.is_nil() {
            builder = builder.dropout(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("unk_token"))?;
        if !value.is_nil() {
            builder = builder.unk_token(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("continuing_subword_prefix"))?;
        if !value.is_nil() {
            builder = builder.continuing_subword_prefix(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("end_of_word_suffix"))?;
        if !value.is_nil() {
            builder = builder.end_of_word_suffix(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("fuse_unk"))?;
        if !value.is_nil() {
            builder = builder.fuse_unk(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("byte_fallback"))?;
        if !value.is_nil() {
            builder = builder.byte_fallback(value.try_convert()?);
        }

        if !kwargs.is_empty() {
            // TODO improve message
            return Err(Error::new(exception::arg_error(), "unknown keyword"));
        }

        builder.build().map(|v| v.into()).map_err(RbError::from)
    }

    pub fn new(vocab: Option<Vocab>, merges: Option<Merges>, kwargs: RHash) -> RbResult<RbModel> {
        let mut builder = BPE::builder();
        if let (Some(vocab), Some(merges)) = (vocab, merges) {
            builder = builder.vocab_and_merges(vocab, merges);
        }
        RbBPE::with_builder(builder, kwargs)
    }

    pub fn from_file(vocab: String, merges: String, kwargs: RHash) -> RbResult<RbModel> {
        let (vocab, merges) = BPE::read_file(&vocab, &merges).map_err(RbError::from)?;

        RbBPE::new(Some(vocab), Some(merges), kwargs)
    }
}

macro_rules! getter {
    ($self: ident, $variant: ident, $($name: tt)+) => {{
        let model = $self.model.write().unwrap();
        if let ModelWrapper::$variant(ref mo) = *model {
            mo.$($name)+
        } else {
            unreachable!()
        }
    }};
}

macro_rules! setter {
    ($self: ident, $variant: ident, $name: ident, $value: expr) => {{
        let mut model = $self.model.write().unwrap();
        if let ModelWrapper::$variant(ref mut mo) = *model {
            mo.$name = $value;
        }
    }};
}

impl RbModel {
    pub fn bpe_dropout(&self) -> Option<f32> {
        getter!(self, BPE, dropout)
    }

    pub fn bpe_set_dropout(&self, dropout: Option<f32>) {
        setter!(self, BPE, dropout, dropout);
    }

    pub fn bpe_unk_token(&self) -> Option<String> {
        getter!(self, BPE, unk_token.clone())
    }

    pub fn bpe_set_unk_token(&self, unk_token: Option<String>) {
        setter!(self, BPE, unk_token, unk_token);
    }

    pub fn bpe_fuse_unk(&self) -> bool {
        getter!(self, BPE, fuse_unk)
    }

    pub fn bpe_set_fuse_unk(&self, fuse_unk: bool) {
        setter!(self, BPE, fuse_unk, fuse_unk);
    }

    pub fn bpe_byte_fallback(&self) -> bool {
        getter!(self, BPE, byte_fallback)
    }

    pub fn bpe_set_byte_fallback(&self, byte_fallback: bool) {
        setter!(self, BPE, byte_fallback, byte_fallback);
    }

    pub fn bpe_continuing_subword_prefix(&self) -> Option<String> {
        getter!(self, BPE, continuing_subword_prefix.clone())
    }

    pub fn bpe_set_continuing_subword_prefix(&self, continuing_subword_prefix: Option<String>) {
        setter!(self, BPE, continuing_subword_prefix, continuing_subword_prefix);
    }

    pub fn bpe_end_of_word_suffix(&self) -> Option<String> {
        getter!(self, BPE, end_of_word_suffix.clone())
    }

    pub fn bpe_set_end_of_word_suffix(&self, end_of_word_suffix: Option<String>) {
        setter!(self, BPE, end_of_word_suffix, end_of_word_suffix);
    }

    pub fn word_level_unk_token(&self) -> String {
        getter!(self, WordLevel, unk_token.clone())
    }

    pub fn word_level_set_unk_token(&self, unk_token: String) {
        setter!(self, WordLevel, unk_token, unk_token);
    }

    pub fn word_piece_unk_token(&self) -> String {
        getter!(self, WordPiece, unk_token.clone())
    }

    pub fn word_piece_set_unk_token(&self, unk_token: String) {
        setter!(self, WordPiece, unk_token, unk_token);
    }

    pub fn word_piece_continuing_subword_prefix(&self) -> String {
        getter!(self, WordPiece, continuing_subword_prefix.clone())
    }

    pub fn word_piece_set_continuing_subword_prefix(&self, continuing_subword_prefix: String) {
        setter!(self, WordPiece, continuing_subword_prefix, continuing_subword_prefix);
    }

    pub fn word_piece_max_input_chars_per_word(&self) -> usize {
        getter!(self, WordPiece, max_input_chars_per_word.clone())
    }

    pub fn word_piece_set_max_input_chars_per_word(&self, max_input_chars_per_word: usize) {
        setter!(self, WordPiece, max_input_chars_per_word, max_input_chars_per_word);
    }
}

pub struct RbUnigram {}

impl RbUnigram {
    fn new(vocab: Option<Vec<(String, f64)>>, unk_id: Option<usize>) -> RbResult<RbModel> {
        match (vocab, unk_id) {
            (Some(vocab), unk_id) => {
                let model = Unigram::from(vocab, unk_id).map_err(RbError::from)?;
                Ok(model.into())
            }
            (None, None) => Ok(Unigram::default().into()),
            _ => Err(Error::new(exception::arg_error(), "`vocab` and `unk_id` must be both specified")),
        }
    }
}

pub struct RbWordLevel {}

impl RbWordLevel {
    pub fn new(vocab: Option<HashMap<String, u32>>, unk_token: Option<String>) -> RbResult<RbModel> {
        let mut builder = WordLevel::builder();
        if let Some(vocab) = vocab {
            builder = builder.vocab(vocab);
        }
        if let Some(unk_token) = unk_token {
            builder = builder.unk_token(unk_token);
        }
        builder.build().map(|v| v.into()).map_err(RbError::from)
    }

    pub fn read_file(vocab: String) -> RbResult<Vocab> {
        WordLevel::read_file(&vocab).map_err(RbError::from)
    }

    pub fn from_file(vocab: String, unk_token: Option<String>) -> RbResult<RbModel> {
        let vocab = WordLevel::read_file(&vocab).map_err(RbError::from)?;

        RbWordLevel::new(Some(vocab), unk_token)
    }
}

pub struct RbWordPiece {}

impl RbWordPiece {
    fn with_builder(mut builder: WordPieceBuilder, kwargs: RHash) -> RbResult<RbModel> {
        let value: Value = kwargs.delete(Symbol::new("unk_token"))?;
        if !value.is_nil() {
            builder = builder.unk_token(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("max_input_chars_per_word"))?;
        if !value.is_nil() {
            builder = builder.max_input_chars_per_word(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("continuing_subword_prefix"))?;
        if !value.is_nil() {
            builder = builder.continuing_subword_prefix(value.try_convert()?);
        }

        if !kwargs.is_empty() {
            // TODO improve message
            return Err(Error::new(exception::arg_error(), "unknown keyword"));
        }

        builder.build().map(|v| v.into()).map_err(RbError::from)
    }

    pub fn new(vocab: Option<HashMap<String, u32>>, kwargs: RHash) -> RbResult<RbModel> {
        let mut builder = WordPiece::builder();
        if let Some(vocab) = vocab {
            builder = builder.vocab(vocab);
        }
        RbWordPiece::with_builder(builder, kwargs)
    }

    pub fn from_file(vocab: String, kwargs: RHash) -> RbResult<RbModel> {
        let vocab = WordPiece::read_file(&vocab).map_err(RbError::from)?;

        RbWordPiece::new(Some(vocab), kwargs)
    }
}

unsafe impl TypedData for RbModel {
    fn class() -> RClass {
        *memoize!(RClass: {
          let class: RClass = crate::models().const_get("Model").unwrap();
          class.undef_alloc_func();
          class
        })
    }

    fn data_type() -> &'static DataType {
        memoize!(DataType: DataTypeBuilder::<RbModel>::new("Tokenizers::Models::Model").build())
    }

    fn class_for(value: &Self) -> RClass {
        match *value.model.read().unwrap() {
            ModelWrapper::BPE(_) => *memoize!(RClass: {
                let class: RClass = crate::models().const_get("BPE").unwrap();
                class.undef_alloc_func();
                class
            }),
            ModelWrapper::Unigram(_) => *memoize!(RClass: {
                let class: RClass = crate::models().const_get("Unigram").unwrap();
                class.undef_alloc_func();
                class
            }),
            ModelWrapper::WordLevel(_) => *memoize!(RClass: {
                let class: RClass = crate::models().const_get("WordLevel").unwrap();
                class.undef_alloc_func();
                class
            }),
            ModelWrapper::WordPiece(_) => *memoize!(RClass: {
                let class: RClass = crate::models().const_get("WordPiece").unwrap();
                class.undef_alloc_func();
                class
            }),
        }
    }
}

pub fn models(module: &RModule) -> RbResult<()> {
    let model = module.define_class("Model", Default::default())?;

    let class = module.define_class("BPE", model)?;
    class.define_singleton_method("_new", function!(RbBPE::new, 3))?;
    class.define_singleton_method("_from_file", function!(RbBPE::from_file, 3))?;
    class.define_method("dropout", method!(RbModel::bpe_dropout, 0))?;
    class.define_method("dropout=", method!(RbModel::bpe_set_dropout, 1))?;
    class.define_method("unk_token", method!(RbModel::bpe_unk_token, 0))?;
    class.define_method("unk_token=", method!(RbModel::bpe_set_unk_token, 1))?;
    class.define_method("continuing_subword_prefix", method!(RbModel::bpe_continuing_subword_prefix, 0))?;
    class.define_method("continuing_subword_prefix=", method!(RbModel::bpe_set_continuing_subword_prefix, 1))?;
    class.define_method("end_of_word_suffix", method!(RbModel::bpe_end_of_word_suffix, 0))?;
    class.define_method("end_of_word_suffix=", method!(RbModel::bpe_set_end_of_word_suffix, 1))?;
    class.define_method("fuse_unk", method!(RbModel::bpe_fuse_unk, 0))?;
    class.define_method("fuse_unk=", method!(RbModel::bpe_set_fuse_unk, 1))?;
    class.define_method("byte_fallback", method!(RbModel::bpe_byte_fallback, 0))?;
    class.define_method("byte_fallback=", method!(RbModel::bpe_set_byte_fallback, 1))?;

    let class = module.define_class("Unigram", model)?;
    class.define_singleton_method("_new", function!(RbUnigram::new, 2))?;

    let class = module.define_class("WordLevel", model)?;
    class.define_singleton_method("_new", function!(RbWordLevel::new, 2))?;
    class.define_singleton_method("_from_file", function!(RbWordLevel::from_file, 2))?;
    class.define_singleton_method("read_file", function!(RbWordLevel::read_file, 1))?;
    class.define_method("unk_token", method!(RbModel::word_level_unk_token, 0))?;
    class.define_method("unk_token=", method!(RbModel::word_level_set_unk_token, 1))?;

    let class = module.define_class("WordPiece", model)?;
    class.define_singleton_method("_new", function!(RbWordPiece::new, 2))?;
    class.define_singleton_method("_from_file", function!(RbWordPiece::from_file, 2))?;
    class.define_method("unk_token", method!(RbModel::word_piece_unk_token, 0))?;
    class.define_method("unk_token=", method!(RbModel::word_piece_set_unk_token, 1))?;
    class.define_method("continuing_subword_prefix", method!(RbModel::word_piece_continuing_subword_prefix, 0))?;
    class.define_method("continuing_subword_prefix=", method!(RbModel::word_piece_set_continuing_subword_prefix, 1))?;
    class.define_method("max_input_chars_per_word", method!(RbModel::word_piece_max_input_chars_per_word, 0))?;
    class.define_method("max_input_chars_per_word=", method!(RbModel::word_piece_set_max_input_chars_per_word, 1))?;

    Ok(())
}
