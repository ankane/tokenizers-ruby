use std::sync::{Arc, RwLock};

use crate::models::RbModel;
use crate::tokenizer::RbAddedToken;
use magnus::{RArray, RHash, Symbol};
use serde::{Deserialize, Serialize};
use tk::models::TrainerWrapper;
use tk::Trainer;

use crate::RbResult;

#[magnus::wrap(class = "Tokenizers::Trainer")]
#[derive(Clone, Deserialize, Serialize)]
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

#[magnus::wrap(class = "Tokenizers::BpeTrainer")]
pub struct RbBpeTrainer {}

impl RbBpeTrainer {
    // TODO error on unknown kwargs
    // TODO return BpeTrainer class
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
