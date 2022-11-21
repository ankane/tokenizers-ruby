use tk::models::bpe::BPE;

use super::{RbError, RbResult};

#[magnus::wrap(class = "Tokenizers::BPE")]
pub struct RbBPE {
    pub model: BPE,
}

impl RbBPE {
    pub fn new(vocab: String, merges: String) -> RbResult<Self> {
        BPE::from_file(&vocab, &merges)
            .unk_token("<unk>".into())
            .end_of_word_suffix("</w>".into())
            .build()
            .map(|v| RbBPE { model: v })
            .map_err(RbError::from)
    }
}
