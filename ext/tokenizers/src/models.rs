use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use crate::trainers::RbTrainer;
use magnus::{RHash, Symbol};
use serde::{Deserialize, Serialize};
use tk::models::bpe::{BpeBuilder, Merges, Vocab, BPE};
use tk::models::ModelWrapper;
use tk::{Model, Token};

use super::{RbError, RbResult};

#[magnus::wrap(class = "Tokenizers::Model")]
#[derive(Clone, Serialize, Deserialize)]
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

#[magnus::wrap(class = "Tokenizers::BPE")]
pub struct RbBPE {}

impl RbBPE {
    // TODO error on unknown kwargs
    fn with_builder(mut builder: BpeBuilder, kwargs: RHash) -> RbResult<RbModel> {
        if let Some(value) = kwargs.get(Symbol::new("unk_token")) {
            builder = builder.unk_token(value.try_convert()?);
        }

        if let Some(value) = kwargs.get(Symbol::new("end_of_word_suffix")) {
            builder = builder.end_of_word_suffix(value.try_convert()?);
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
