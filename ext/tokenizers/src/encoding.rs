use tk::Encoding;

#[magnus::wrap(class = "Tokenizers::Encoding")]
pub struct RbEncoding {
    pub encoding: Encoding,
}

impl RbEncoding {
    pub fn n_sequences(&self) -> usize {
        self.encoding.n_sequences()
    }

    pub fn ids(&self) -> Vec<u32> {
        self.encoding.get_ids().to_vec()
    }

    pub fn tokens(&self) -> Vec<String> {
        self.encoding.get_tokens().to_vec()
    }

    pub fn word_ids(&self) -> Vec<Option<u32>> {
        self.encoding.get_word_ids().to_vec()
    }

    pub fn sequence_ids(&self) -> Vec<Option<usize>> {
        self.encoding.get_sequence_ids()
    }

    pub fn type_ids(&self) -> Vec<u32> {
        self.encoding.get_type_ids().to_vec()
    }

    pub fn offsets(&self) -> Vec<(usize, usize)> {
        self.encoding.get_offsets().to_vec()
    }

    pub fn special_tokens_mask(&self) -> Vec<u32> {
        self.encoding.get_special_tokens_mask().to_vec()
    }

    pub fn attention_mask(&self) -> Vec<u32> {
        self.encoding.get_attention_mask().to_vec()
    }
}
