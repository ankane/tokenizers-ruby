use tk::normalizers::BertNormalizer;

#[magnus::wrap(class = "Tokenizers::Normalizer")]
pub struct RbNormalizer {}

#[magnus::wrap(class = "Tokenizers::BertNormalizer")]
pub struct RbBertNormalizer {
    pub normalizer: BertNormalizer,
}

impl RbBertNormalizer {
    pub fn new() -> Self {
        RbBertNormalizer {
            normalizer: BertNormalizer::default(),
        }
    }
}
