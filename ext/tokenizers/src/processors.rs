use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tk::processors::PostProcessorWrapper;
use tk::{Encoding, PostProcessor};

#[magnus::wrap(class = "Tokenizers::PostProcessor")]
#[derive(Clone, Deserialize, Serialize)]
pub struct RbPostProcessor {
    #[serde(flatten)]
    pub processor: Arc<PostProcessorWrapper>,
}

impl PostProcessor for RbPostProcessor {
    fn added_tokens(&self, is_pair: bool) -> usize {
        self.processor.added_tokens(is_pair)
    }

    fn process_encodings(
        &self,
        encodings: Vec<Encoding>,
        add_special_tokens: bool,
    ) -> tk::Result<Vec<Encoding>> {
        self.processor
            .process_encodings(encodings, add_special_tokens)
    }
}
