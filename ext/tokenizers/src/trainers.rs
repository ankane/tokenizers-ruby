use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use crate::models::RbModel;
use crate::tokenizer::RbAddedToken;
use magnus::typed_data::DataTypeBuilder;
use magnus::{
    exception, function, memoize, method, Class, DataType, DataTypeFunctions, Error, Module, Object,
    RArray, RClass, RHash, RModule, Symbol, TypedData, Value,
};
use serde::{Deserialize, Serialize};
use tk::models::TrainerWrapper;
use tk::Trainer;

use super::RbResult;

#[derive(DataTypeFunctions, Clone, Deserialize, Serialize)]
pub struct RbTrainer {
    #[serde(flatten)]
    pub trainer: Arc<RwLock<TrainerWrapper>>,
}

impl Trainer for RbTrainer {
    type Model = RbModel;

    fn should_show_progress(&self) -> bool {
        self.trainer.read().unwrap().should_show_progress()
    }

    fn train(&self, model: &mut RbModel) -> tk::Result<Vec<tk::AddedToken>> {
        self.trainer
            .read()
            .unwrap()
            .train(&mut model.model.write().unwrap())
    }

    fn feed<I, S, F>(&mut self, iterator: I, process: F) -> tk::Result<()>
    where
        I: Iterator<Item = S> + Send,
        S: AsRef<str> + Send,
        F: Fn(&str) -> tk::Result<Vec<String>> + Sync,
    {
        self.trainer.write().unwrap().feed(iterator, process)
    }
}

macro_rules! getter {
    ($self: ident, $variant: ident, $($name: tt)+) => {{
        if let TrainerWrapper::$variant(ref trainer) = *$self.trainer.read().unwrap() {
            trainer.$($name)+
        } else {
            unreachable!()
        }
    }};
}

macro_rules! setter {
    ($self: ident, $variant: ident, $name: ident, $value: expr) => {{
        if let TrainerWrapper::$variant(ref mut trainer) = *$self.trainer.write().unwrap() {
            trainer.$name = $value;
        }
    }};
    ($self: ident, $variant: ident, @$name: ident, $value: expr) => {{
        if let TrainerWrapper::$variant(ref mut trainer) = *$self.trainer.write().unwrap() {
            trainer.$name($value);
        }
    }};
}

impl RbTrainer {

    fn bpe_trainer_vocab_size(&self) -> usize {
        getter!(self, BpeTrainer, vocab_size)
    }

    fn bpe_trainer_set_vocab_size(&self, vocab_size: usize) {
        setter!(self, BpeTrainer, vocab_size, vocab_size);
    }

    fn bpe_trainer_min_frequency(&self) -> u32 {
        getter!(self, BpeTrainer, min_frequency)
    }

    fn bpe_trainer_set_min_frequency(&self, freq: u32) {
        setter!(self, BpeTrainer, min_frequency, freq);
    }

    fn bpe_trainer_show_progress(&self) -> bool {
        getter!(self, BpeTrainer, show_progress)
    }

    fn bpe_trainer_set_show_progress(&self, show_progress: bool) {
        setter!(self, BpeTrainer, show_progress, show_progress);
    }

    fn bpe_trainer_special_tokens(&self) -> Vec<String> {
        getter!(
            self,
            BpeTrainer,
            special_tokens
                .iter()
                .map(|tok| tok.content.clone())
                .collect()
        )
    }

    fn bpe_trainer_set_special_tokens(&self, special_tokens: RArray) -> RbResult<()> {
        setter!(
            self,
            BpeTrainer,
            special_tokens,
            special_tokens
                .each()
                .map(|token| {
                    if let Ok(content) = token?.try_convert::<String>() {
                        Ok(RbAddedToken::from(content, Some(true)).get_token())
                    } else {
                        todo!()
                    }
                })
                .collect::<RbResult<Vec<_>>>()?
        );
        Ok(())
    }

    fn bpe_trainer_limit_alphabet(&self) -> Option<usize> {
        getter!(self, BpeTrainer, limit_alphabet)
    }

    fn bpe_trainer_set_limit_alphabet(&self, limit: Option<usize>) {
        setter!(self, BpeTrainer, limit_alphabet, limit);
    }

    fn bpe_trainer_initial_alphabet(&self) -> Vec<String> {
        getter!(
            self,
            BpeTrainer,
            initial_alphabet.iter().map(|c| c.to_string()).collect()
        )
    }

    fn bpe_trainer_set_initial_alphabet(&self, alphabet: Vec<char>) {
        setter!(
            self,
            BpeTrainer,
            initial_alphabet,
            alphabet.into_iter().map(|c| c).collect()
        );
    }

    fn bpe_trainer_continuing_subword_prefix(&self) -> Option<String> {
        getter!(self, BpeTrainer, continuing_subword_prefix.clone())
    }

    fn bpe_trainer_set_continuing_subword_prefix(&self, prefix: Option<String>) {
        setter!(self, BpeTrainer, continuing_subword_prefix, prefix);
    }

    fn bpe_trainer_end_of_word_suffix(&self) -> Option<String> {
        getter!(self, BpeTrainer, end_of_word_suffix.clone())
    }

    fn bpe_trainer_set_end_of_word_suffix(&self, suffix: Option<String>) {
        setter!(self, BpeTrainer, end_of_word_suffix, suffix);
    }

    fn unigram_trainer_vocab_size(&self) -> u32 {
        getter!(self, UnigramTrainer, vocab_size)
    }

    fn unigram_trainer_set_vocab_size(&self, vocab_size: u32) {
        setter!(self, UnigramTrainer, vocab_size, vocab_size);
    }

    fn unigram_trainer_show_progress(&self) -> bool {
        getter!(self, UnigramTrainer, show_progress)
    }

    fn unigram_trainer_set_show_progress(&self, show_progress: bool) {
        setter!(self, UnigramTrainer, show_progress, show_progress);
    }

    fn unigram_trainer_special_tokens(&self) -> Vec<String> {
        getter!(
            self,
            UnigramTrainer,
            special_tokens
                .iter()
                .map(|tok| tok.content.clone())
                .collect()
        )
    }

    fn unigram_trainer_set_special_tokens(&self, special_tokens: RArray) -> RbResult<()> {
        setter!(
            self,
            UnigramTrainer,
            special_tokens,
            special_tokens
                .each()
                .map(|token| {
                    if let Ok(content) = token?.try_convert::<String>() {
                        Ok(RbAddedToken::from(content, Some(true)).get_token())
                    } else {
                        todo!()
                    }
                })
                .collect::<RbResult<Vec<_>>>()?
        );
        Ok(())
    }

    fn unigram_trainer_initial_alphabet(&self) -> Vec<String> {
        getter!(
            self,
            UnigramTrainer,
            initial_alphabet.iter().map(|c| c.to_string()).collect()
        )
    }

    fn unigram_trainer_set_initial_alphabet(&self, alphabet: Vec<char>) {
        setter!(
            self,
            UnigramTrainer,
            initial_alphabet,
            alphabet.into_iter().map(|c| c).collect()
        );
    }

    fn word_level_trainer_vocab_size(&self) -> usize {
        getter!(self, WordLevelTrainer, vocab_size)
    }

    fn word_level_trainer_set_vocab_size(&self, vocab_size: usize) {
        setter!(self, WordLevelTrainer, vocab_size, vocab_size);
    }

    fn word_level_trainer_min_frequency(&self) -> u32 {
        getter!(self, WordLevelTrainer, min_frequency)
    }

    fn word_level_trainer_set_min_frequency(&self, freq: u32) {
        setter!(self, WordLevelTrainer, min_frequency, freq);
    }

    fn word_level_trainer_show_progress(&self) -> bool {
        getter!(self, WordLevelTrainer, show_progress)
    }

    fn word_level_trainer_set_show_progress(&self, show_progress: bool) {
        setter!(self, WordLevelTrainer, show_progress, show_progress);
    }

    fn word_level_trainer_special_tokens(&self) -> Vec<String> {
        getter!(
            self,
            WordLevelTrainer,
            special_tokens
                .iter()
                .map(|tok| tok.content.clone())
                .collect()
        )
    }

    fn word_level_trainer_set_special_tokens(&self, special_tokens: RArray) -> RbResult<()> {
        setter!(
            self,
            WordLevelTrainer,
            special_tokens,
            special_tokens
                .each()
                .map(|token| {
                    if let Ok(content) = token?.try_convert::<String>() {
                        Ok(RbAddedToken::from(content, Some(true)).get_token())
                    } else {
                        todo!()
                    }
                })
                .collect::<RbResult<Vec<_>>>()?
        );
        Ok(())
    }

    fn word_piece_trainer_vocab_size(&self) -> usize {
        getter!(self, WordPieceTrainer, vocab_size())
    }

    fn word_piece_trainer_set_vocab_size(&self, vocab_size: usize) {
        setter!(self, WordPieceTrainer, @set_vocab_size, vocab_size);
    }

    fn word_piece_trainer_min_frequency(&self) -> u32 {
        getter!(self, WordPieceTrainer, min_frequency())
    }

    fn word_piece_trainer_set_min_frequency(&self, freq: u32) {
        setter!(self, WordPieceTrainer, @set_min_frequency, freq);
    }

    fn word_piece_trainer_show_progress(&self) -> bool {
        getter!(self, WordPieceTrainer, show_progress())
    }

    fn word_piece_trainer_set_show_progress(&self, show_progress: bool) {
        setter!(self, WordPieceTrainer, @set_show_progress, show_progress);
    }

    fn word_piece_trainer_special_tokens(&self) -> Vec<String> {
        getter!(
            self,
            WordPieceTrainer,
            special_tokens()
                .iter()
                .map(|tok| tok.content.clone())
                .collect()
        )
    }

    fn word_piece_trainer_set_special_tokens(&self, special_tokens: RArray) -> RbResult<()> {
        setter!(
            self,
            WordPieceTrainer,
            @set_special_tokens,
            special_tokens
                .each()
                .map(|token| {
                    if let Ok(content) = token?.try_convert::<String>() {
                        Ok(RbAddedToken::from(content, Some(true)).get_token())
                    } else {
                        todo!()
                    }
                })
                .collect::<RbResult<Vec<_>>>()?
        );
        Ok(())
    }

    fn word_piece_trainer_limit_alphabet(&self) -> Option<usize> {
        getter!(self, WordPieceTrainer, limit_alphabet())
    }

    fn word_piece_trainer_set_limit_alphabet(&self, limit: Option<usize>) {
        setter!(self, WordPieceTrainer, @set_limit_alphabet, limit);
    }

    fn word_piece_trainer_initial_alphabet(&self) -> Vec<String> {
        getter!(
            self,
            WordPieceTrainer,
            initial_alphabet().iter().map(|c| c.to_string()).collect()
        )
    }

    fn word_piece_trainer_set_initial_alphabet(&self, alphabet: Vec<char>) {
        setter!(
            self,
            WordPieceTrainer,
            @set_initial_alphabet,
            alphabet.into_iter().map(|c| c).collect()
        );
    }

    fn word_piece_trainer_continuing_subword_prefix(&self) -> Option<String> {
        getter!(self, WordPieceTrainer, continuing_subword_prefix().clone())
    }

    fn word_piece_trainer_set_continuing_subword_prefix(&self, prefix: Option<String>) {
        setter!(self, WordPieceTrainer, @set_continuing_subword_prefix, prefix);
    }

    fn word_piece_trainer_end_of_word_suffix(&self) -> Option<String> {
        getter!(self, WordPieceTrainer, end_of_word_suffix().clone())
    }

    fn word_piece_trainer_set_end_of_word_suffix(&self, suffix: Option<String>) {
        setter!(self, WordPieceTrainer, @set_end_of_word_suffix, suffix);
    }
}

impl<I> From<I> for RbTrainer
where
    I: Into<TrainerWrapper>,
{
    fn from(trainer: I) -> Self {
        RbTrainer {
            trainer: Arc::new(RwLock::new(trainer.into())),
        }
    }
}

pub struct RbBpeTrainer {}

impl RbBpeTrainer {
    pub fn new(kwargs: RHash) -> RbResult<RbTrainer> {
        let mut builder = tk::models::bpe::BpeTrainer::builder();

        let value: Value = kwargs.delete(Symbol::new("special_tokens"))?;
        if !value.is_nil() {
            builder = builder.special_tokens(
                value
                    .try_convert::<RArray>()?
                    .each()
                    .map(|token| {
                        if let Ok(content) = token?.try_convert::<String>() {
                            Ok(RbAddedToken::from(content, Some(true)).get_token())
                        } else {
                            todo!()
                        }
                    })
                    .collect::<RbResult<Vec<_>>>()?,
            );
        }

        let value: Value = kwargs.delete(Symbol::new("initial_alphabet"))?;
        if !value.is_nil() {
            let arr = value.try_convert::<Vec<char>>()?;
            let set: HashSet<char> = HashSet::from_iter(arr);
            builder = builder.initial_alphabet(set);
        }

        let value: Value = kwargs.delete(Symbol::new("vocab_size"))?;
        if !value.is_nil() {
            builder = builder.vocab_size(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("min_frequency"))?;
        if !value.is_nil() {
            builder = builder.min_frequency(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("show_progress"))?;
        if !value.is_nil() {
            builder = builder.show_progress(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("limit_alphabet"))?;
        if !value.is_nil() {
            builder = builder.limit_alphabet(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("continuing_subword_prefix"))?;
        if !value.is_nil() {
            builder = builder.continuing_subword_prefix(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("end_of_word_suffix"))?;
        if !value.is_nil() {
            builder = builder.end_of_word_suffix(value.try_convert()?);
        }

        if !kwargs.is_empty() {
            // TODO improve message
            return Err(Error::new(exception::arg_error(), "unknown keyword"));
        }

        Ok(builder.build().into())
    }
}

pub struct RbUnigramTrainer {}

impl RbUnigramTrainer {
    pub fn new(kwargs: RHash) -> RbResult<RbTrainer> {
        let mut builder = tk::models::unigram::UnigramTrainer::builder();

        let value: Value = kwargs.delete(Symbol::new("special_tokens"))?;
        if !value.is_nil() {
            builder.special_tokens(
                value
                    .try_convert::<RArray>()?
                    .each()
                    .map(|token| {
                        if let Ok(content) = token?.try_convert::<String>() {
                            Ok(RbAddedToken::from(content, Some(true)).get_token())
                        } else {
                            todo!()
                        }
                    })
                    .collect::<RbResult<Vec<_>>>()?,
            );
        }

        let value: Value = kwargs.delete(Symbol::new("initial_alphabet"))?;
        if !value.is_nil() {
            let arr = value.try_convert::<Vec<char>>()?;
            let set: HashSet<char> = HashSet::from_iter(arr);
            builder.initial_alphabet(set);
        }

        let value: Value = kwargs.delete(Symbol::new("vocab_size"))?;
        if !value.is_nil() {
            builder.vocab_size(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("show_progress"))?;
        if !value.is_nil() {
            builder.show_progress(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("n_sub_iterations"))?;
        if !value.is_nil() {
            builder.n_sub_iterations(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("unk_token"))?;
        if !value.is_nil() {
            builder.unk_token(Some(value.try_convert()?));
        }

        let value: Value = kwargs.delete(Symbol::new("max_piece_length"))?;
        if !value.is_nil() {
            builder.max_piece_length(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("seed_size"))?;
        if !value.is_nil() {
            builder.seed_size(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("shrinking_factor"))?;
        if !value.is_nil() {
            builder.shrinking_factor(value.try_convert()?);
        }

        if !kwargs.is_empty() {
            // TODO improve message
            return Err(Error::new(exception::arg_error(), "unknown keyword"));
        }

        let trainer = builder.build().map_err(|_| { Error::new(exception::arg_error(), "Cannot build UnigramTrainer") })?;
        Ok(trainer.into())
    }
}

pub struct RbWordLevelTrainer {}

impl RbWordLevelTrainer {
    pub fn new(kwargs: RHash) -> RbResult<RbTrainer> {
        let mut builder = tk::models::wordlevel::WordLevelTrainer::builder();

        let value: Value = kwargs.delete(Symbol::new("special_tokens"))?;
        if !value.is_nil() {
            builder.special_tokens(
                value
                    .try_convert::<RArray>()?
                    .each()
                    .map(|token| {
                        if let Ok(content) = token?.try_convert::<String>() {
                            Ok(RbAddedToken::from(content, Some(true)).get_token())
                        } else {
                            todo!()
                        }
                    })
                    .collect::<RbResult<Vec<_>>>()?,
            );
        }

        let value: Value = kwargs.delete(Symbol::new("vocab_size"))?;
        if !value.is_nil() {
            builder.vocab_size(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("min_frequency"))?;
        if !value.is_nil() {
            builder.min_frequency(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("show_progress"))?;
        if !value.is_nil() {
            builder.show_progress(value.try_convert()?);
        }

        Ok(builder.build().expect("WordLevelTrainerBuilder cannot fail").into())
    }
}

pub struct RbWordPieceTrainer {}

impl RbWordPieceTrainer {
    pub fn new(kwargs: RHash) -> RbResult<RbTrainer> {
        let mut builder = tk::models::wordpiece::WordPieceTrainer::builder();

        let value: Value = kwargs.delete(Symbol::new("special_tokens"))?;
        if !value.is_nil() {
            builder = builder.special_tokens(
                value
                    .try_convert::<RArray>()?
                    .each()
                    .map(|token| {
                        if let Ok(content) = token?.try_convert::<String>() {
                            Ok(RbAddedToken::from(content, Some(true)).get_token())
                        } else {
                            todo!()
                        }
                    })
                    .collect::<RbResult<Vec<_>>>()?,
            );
        }

        let value: Value = kwargs.delete(Symbol::new("initial_alphabet"))?;
        if !value.is_nil() {
            let arr = value.try_convert::<Vec<char>>()?;
            let set: HashSet<char> = HashSet::from_iter(arr);
            builder = builder.initial_alphabet(set);
        }

        let value: Value = kwargs.delete(Symbol::new("vocab_size"))?;
        if !value.is_nil() {
            builder = builder.vocab_size(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("min_frequency"))?;
        if !value.is_nil() {
            builder = builder.min_frequency(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("show_progress"))?;
        if !value.is_nil() {
            builder = builder.show_progress(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("limit_alphabet"))?;
        if !value.is_nil() {
            builder = builder.limit_alphabet(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("continuing_subword_prefix"))?;
        if !value.is_nil() {
            builder = builder.continuing_subword_prefix(value.try_convert()?);
        }

        let value: Value = kwargs.delete(Symbol::new("end_of_word_suffix"))?;
        if !value.is_nil() {
            builder = builder.end_of_word_suffix(value.try_convert()?);
        }

        if !kwargs.is_empty() {
            // TODO improve message
            return Err(Error::new(exception::arg_error(), "unknown keyword"));
        }

        Ok(builder.build().into())
    }
}

unsafe impl TypedData for RbTrainer {
    fn class() -> RClass {
        *memoize!(RClass: {
          let class: RClass = crate::trainers().const_get("Trainer").unwrap();
          class.undef_alloc_func();
          class
        })
    }

    fn data_type() -> &'static DataType {
        memoize!(DataType: DataTypeBuilder::<RbTrainer>::new("Tokenizers::Trainers::Trainer").build())
    }

    fn class_for(value: &Self) -> RClass {
        match *value.trainer.read().unwrap() {
            TrainerWrapper::BpeTrainer(_) => *memoize!(RClass: {
                let class: RClass = crate::trainers().const_get("BpeTrainer").unwrap();
                class.undef_alloc_func();
                class
            }),
            TrainerWrapper::UnigramTrainer(_) => *memoize!(RClass: {
                let class: RClass = crate::trainers().const_get("UnigramTrainer").unwrap();
                class.undef_alloc_func();
                class
            }),
            TrainerWrapper::WordLevelTrainer(_) => *memoize!(RClass: {
                let class: RClass = crate::trainers().const_get("WordLevelTrainer").unwrap();
                class.undef_alloc_func();
                class
            }),
            TrainerWrapper::WordPieceTrainer(_) => *memoize!(RClass: {
                let class: RClass = crate::trainers().const_get("WordPieceTrainer").unwrap();
                class.undef_alloc_func();
                class
            }),
        }
    }
}

pub fn trainers(module: &RModule) -> RbResult<()> {
    let trainer = module.define_class("Trainer", Default::default())?;

    let class = module.define_class("BpeTrainer", trainer)?;
    class.define_singleton_method("_new", function!(RbBpeTrainer::new, 1))?;
    class.define_method("vocab_size", method!(RbTrainer::bpe_trainer_vocab_size, 0))?;
    class.define_method("vocab_size=", method!(RbTrainer::bpe_trainer_set_vocab_size, 1))?;
    class.define_method("min_frequency", method!(RbTrainer::bpe_trainer_min_frequency, 0))?;
    class.define_method("min_frequency=", method!(RbTrainer::bpe_trainer_set_min_frequency, 1))?;
    class.define_method("show_progress", method!(RbTrainer::bpe_trainer_show_progress, 0))?;
    class.define_method("show_progress=", method!(RbTrainer::bpe_trainer_set_show_progress, 1))?;
    class.define_method("special_tokens", method!(RbTrainer::bpe_trainer_special_tokens, 0))?;
    class.define_method("special_tokens=", method!(RbTrainer::bpe_trainer_set_special_tokens, 1))?;
    class.define_method("limit_alphabet", method!(RbTrainer::bpe_trainer_limit_alphabet, 0))?;
    class.define_method("limit_alphabet=", method!(RbTrainer::bpe_trainer_set_limit_alphabet, 1))?;
    class.define_method("initial_alphabet", method!(RbTrainer::bpe_trainer_initial_alphabet, 0))?;
    class.define_method("initial_alphabet=", method!(RbTrainer::bpe_trainer_set_initial_alphabet, 1))?;
    class.define_method("continuing_subword_prefix", method!(RbTrainer::bpe_trainer_continuing_subword_prefix, 0))?;
    class.define_method("continuing_subword_prefix=", method!(RbTrainer::bpe_trainer_set_continuing_subword_prefix, 1))?;
    class.define_method("end_of_word_suffix", method!(RbTrainer::bpe_trainer_end_of_word_suffix, 0))?;
    class.define_method("end_of_word_suffix=", method!(RbTrainer::bpe_trainer_set_end_of_word_suffix, 1))?;

    let class = module.define_class("UnigramTrainer", trainer)?;
    class.define_singleton_method("_new", function!(RbUnigramTrainer::new, 1))?;
    class.define_method("vocab_size", method!(RbTrainer::unigram_trainer_vocab_size, 0))?;
    class.define_method("vocab_size=", method!(RbTrainer::unigram_trainer_set_vocab_size, 1))?;
    class.define_method("show_progress", method!(RbTrainer::unigram_trainer_show_progress, 0))?;
    class.define_method("show_progress=", method!(RbTrainer::unigram_trainer_set_show_progress, 1))?;
    class.define_method("special_tokens", method!(RbTrainer::unigram_trainer_special_tokens, 0))?;
    class.define_method("special_tokens=", method!(RbTrainer::unigram_trainer_set_special_tokens, 1))?;
    class.define_method("initial_alphabet", method!(RbTrainer::unigram_trainer_initial_alphabet, 0))?;
    class.define_method("initial_alphabet=", method!(RbTrainer::unigram_trainer_set_initial_alphabet, 1))?;

    let class = module.define_class("WordLevelTrainer", trainer)?;
    class.define_singleton_method("_new", function!(RbWordLevelTrainer::new, 1))?;
    class.define_method("vocab_size", method!(RbTrainer::word_level_trainer_vocab_size, 0))?;
    class.define_method("vocab_size=", method!(RbTrainer::word_level_trainer_set_vocab_size, 1))?;
    class.define_method("min_frequency", method!(RbTrainer::word_level_trainer_min_frequency, 0))?;
    class.define_method("min_frequency=", method!(RbTrainer::word_level_trainer_set_min_frequency, 1))?;
    class.define_method("show_progress", method!(RbTrainer::word_level_trainer_show_progress, 0))?;
    class.define_method("show_progress=", method!(RbTrainer::word_level_trainer_set_show_progress, 1))?;
    class.define_method("special_tokens", method!(RbTrainer::word_level_trainer_special_tokens, 0))?;
    class.define_method("special_tokens=", method!(RbTrainer::word_level_trainer_set_special_tokens, 1))?;

    let class = module.define_class("WordPieceTrainer", trainer)?;
    class.define_singleton_method("_new", function!(RbWordPieceTrainer::new, 1))?;
    class.define_method("vocab_size", method!(RbTrainer::word_piece_trainer_vocab_size, 0))?;
    class.define_method("vocab_size=", method!(RbTrainer::word_piece_trainer_set_vocab_size, 1))?;
    class.define_method("min_frequency", method!(RbTrainer::word_piece_trainer_min_frequency, 0))?;
    class.define_method("min_frequency=", method!(RbTrainer::word_piece_trainer_set_min_frequency, 1))?;
    class.define_method("show_progress", method!(RbTrainer::word_piece_trainer_show_progress, 0))?;
    class.define_method("show_progress=", method!(RbTrainer::word_piece_trainer_set_show_progress, 1))?;
    class.define_method("special_tokens", method!(RbTrainer::word_piece_trainer_special_tokens, 0))?;
    class.define_method("special_tokens=", method!(RbTrainer::word_piece_trainer_set_special_tokens, 1))?;
    class.define_method("limit_alphabet", method!(RbTrainer::word_piece_trainer_limit_alphabet, 0))?;
    class.define_method("limit_alphabet=", method!(RbTrainer::word_piece_trainer_set_limit_alphabet, 1))?;
    class.define_method("initial_alphabet", method!(RbTrainer::word_piece_trainer_initial_alphabet, 0))?;
    class.define_method("initial_alphabet=", method!(RbTrainer::word_piece_trainer_set_initial_alphabet, 1))?;
    class.define_method("continuing_subword_prefix", method!(RbTrainer::word_piece_trainer_continuing_subword_prefix, 0))?;
    class.define_method("continuing_subword_prefix=", method!(RbTrainer::word_piece_trainer_set_continuing_subword_prefix, 1))?;
    class.define_method("end_of_word_suffix", method!(RbTrainer::word_piece_trainer_end_of_word_suffix, 0))?;
    class.define_method("end_of_word_suffix=", method!(RbTrainer::word_piece_trainer_set_end_of_word_suffix, 1))?;

    Ok(())
}
