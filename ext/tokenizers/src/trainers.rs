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
            _ => todo!(),
        }
    }
}

pub fn trainers(module: &RModule) -> RbResult<()> {
    let trainer = module.define_class("Trainer", Default::default())?;

    let class = module.define_class("BpeTrainer", trainer)?;
    class.define_singleton_method("_new", function!(RbBpeTrainer::new, 1))?;

    Ok(())
}
