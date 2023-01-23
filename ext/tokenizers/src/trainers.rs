use magnus::RHash;

#[magnus::wrap(class = "Tokenizers::Trainer")]
pub struct RbTrainer {}

#[magnus::wrap(class = "Tokenizers::BpeTrainer")]
pub struct RbBpeTrainer {}

impl RbBpeTrainer {
    pub fn new(_kwargs: RHash) -> Self {
        Self {}
    }
}
