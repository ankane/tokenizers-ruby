use tk::pre_tokenizers::bert::BertPreTokenizer;

#[magnus::wrap(class = "Tokenizers::BertPreTokenizer")]
pub struct RbBertPreTokenizer {
    pub pretok: BertPreTokenizer,
}

impl RbBertPreTokenizer {
    pub fn new() -> Self {
        RbBertPreTokenizer {
            pretok: BertPreTokenizer,
        }
    }
}
