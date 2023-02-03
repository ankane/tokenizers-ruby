use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use crate::models::RbModel;
use crate::tokenizer::RbAddedToken;
use magnus::typed_data::DataTypeBuilder;
use magnus::{
    exception, function, memoize, Class, DataType, DataTypeFunctions, Error, Module, Object,
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
            builder = builder.vocab_size(value.try_convert::<usize>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("min_frequency"))?;
        if !value.is_nil() {
            builder = builder.min_frequency(value.try_convert::<u32>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("show_progress"))?;
        if !value.is_nil() {
            builder = builder.show_progress(value.try_convert::<bool>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("limit_alphabet"))?;
        if !value.is_nil() {
            builder = builder.limit_alphabet(value.try_convert::<usize>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("continuing_subword_prefix"))?;
        if !value.is_nil() {
            builder = builder.continuing_subword_prefix(value.try_convert::<String>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("end_of_word_suffix"))?;
        if !value.is_nil() {
            builder = builder.end_of_word_suffix(value.try_convert::<String>()?);
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
            builder.vocab_size(value.try_convert::<u32>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("show_progress"))?;
        if !value.is_nil() {
            builder.show_progress(value.try_convert::<bool>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("n_sub_iterations"))?;
        if !value.is_nil() {
            builder.n_sub_iterations(value.try_convert::<u32>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("unk_token"))?;
        if !value.is_nil() {
            builder.unk_token(Some(value.try_convert::<String>()?));
        }

        let value: Value = kwargs.delete(Symbol::new("max_piece_length"))?;
        if !value.is_nil() {
            builder.max_piece_length(value.try_convert::<usize>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("seed_size"))?;
        if !value.is_nil() {
            builder.seed_size(value.try_convert::<usize>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("shrinking_factor"))?;
        if !value.is_nil() {
            builder.shrinking_factor(value.try_convert::<f64>()?);
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
            builder.vocab_size(value.try_convert::<usize>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("min_frequency"))?;
        if !value.is_nil() {
            builder.min_frequency(value.try_convert::<u32>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("show_progress"))?;
        if !value.is_nil() {
            builder.show_progress(value.try_convert::<bool>()?);
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
            builder = builder.vocab_size(value.try_convert::<usize>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("min_frequency"))?;
        if !value.is_nil() {
            builder = builder.min_frequency(value.try_convert::<u32>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("show_progress"))?;
        if !value.is_nil() {
            builder = builder.show_progress(value.try_convert::<bool>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("limit_alphabet"))?;
        if !value.is_nil() {
            builder = builder.limit_alphabet(value.try_convert::<usize>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("continuing_subword_prefix"))?;
        if !value.is_nil() {
            builder = builder.continuing_subword_prefix(value.try_convert::<String>()?);
        }

        let value: Value = kwargs.delete(Symbol::new("end_of_word_suffix"))?;
        if !value.is_nil() {
            builder = builder.end_of_word_suffix(value.try_convert::<String>()?);
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

    let class = module.define_class("UnigramTrainer", trainer)?;
    class.define_singleton_method("_new", function!(RbUnigramTrainer::new, 1))?;

    let class = module.define_class("WordLevelTrainer", trainer)?;
    class.define_singleton_method("_new", function!(RbWordLevelTrainer::new, 1))?;

    let class = module.define_class("WordPieceTrainer", trainer)?;
    class.define_singleton_method("_new", function!(RbWordPieceTrainer::new, 1))?;

    Ok(())
}
