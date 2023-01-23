use magnus::RHash;

#[magnus::wrap(class = "Tokenizers::BpeTrainer")]
pub struct RbBpeTrainer {}

impl RbBpeTrainer {
    pub fn new(kwargs: RHash) -> Self {
        Self {}
    }
}
