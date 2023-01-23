use magnus::{RHash, Symbol};
use tk::models::bpe::{BpeBuilder, Merges, Vocab, BPE};

use super::{RbError, RbResult};

#[magnus::wrap(class = "Tokenizers::BPE")]
pub struct RbBPE {
    pub model: BPE,
}

impl RbBPE {
    // TODO handle unknown kwargs
    fn with_builder(mut builder: BpeBuilder, kwargs: RHash) -> RbResult<Self> {
        if let Some(value) = kwargs.get(Symbol::new("unk_token")) {
            builder = builder.unk_token(value.try_convert()?);
        }

        if let Some(value) = kwargs.get(Symbol::new("end_of_word_suffix")) {
            builder = builder.end_of_word_suffix(value.try_convert()?);
        }

        builder
            .build()
            .map(|v| RbBPE { model: v })
            .map_err(RbError::from)
    }

    pub fn new(vocab: Option<Vocab>, merges: Option<Merges>, kwargs: RHash) -> RbResult<Self> {
        let mut builder = BPE::builder();
        if let (Some(vocab), Some(merges)) = (vocab, merges) {
            builder = builder.vocab_and_merges(vocab, merges);
        }
        RbBPE::with_builder(builder, kwargs)
    }

    pub fn from_file(vocab: String, merges: String, kwargs: RHash) -> RbResult<Self> {
        let (vocab, merges) = BPE::read_file(&vocab, &merges).map_err(RbError::from)?;

        RbBPE::new(Some(vocab), Some(merges), kwargs)
    }
}
