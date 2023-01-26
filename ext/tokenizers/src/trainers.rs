use std::sync::{Arc, RwLock};

use crate::models::RbModel;
use crate::tokenizer::RbAddedToken;
use magnus::typed_data::DataTypeBuilder;
use magnus::{
    memoize, Class, DataType, DataTypeFunctions, Module, RArray, RClass, RHash, Symbol, TypedData,
};
use serde::{Deserialize, Serialize};
use tk::models::TrainerWrapper;
use tk::Trainer;

use super::{module, RbResult};

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
    // TODO error on unknown kwargs
    pub fn new(kwargs: RHash) -> RbResult<RbTrainer> {
        let mut builder = tk::models::bpe::BpeTrainer::builder();

        if let Some(value) = kwargs.get(Symbol::new("special_tokens")) {
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

        Ok(builder.build().into())
    }
}

unsafe impl TypedData for RbTrainer {
    fn class() -> RClass {
        *memoize!(RClass: {
          let class: RClass = module().const_get("Trainer").unwrap();
          class.undef_alloc_func();
          class
        })
    }

    fn data_type() -> &'static DataType {
        memoize!(DataType: DataTypeBuilder::<RbTrainer>::new("Tokenizers::Trainer").build())
    }

    fn class_for(value: &Self) -> RClass {
        match *value.trainer.read().unwrap() {
            TrainerWrapper::BpeTrainer(_) => *memoize!(RClass: {
                let class: RClass = module().const_get("BpeTrainer").unwrap();
                class.undef_alloc_func();
                class
            }),
            _ => Self::class(),
        }
    }
}
