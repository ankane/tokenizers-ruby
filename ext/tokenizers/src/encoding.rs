use tk::Encoding;

#[magnus::wrap(class = "Tokenizers::Encoding")]
#[repr(transparent)]
pub struct RbEncoding {
    pub encoding: Encoding,
}

impl From<Encoding> for RbEncoding {
    fn from(v: Encoding) -> Self {
        Self { encoding: v }
    }
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

    pub fn overflowing(&self) -> Vec<Self> {
        self.encoding
            .get_overflowing()
            .clone()
            .into_iter()
            .map(|e| e.into())
            .collect()
    }

    pub fn word_to_tokens(&self, word_index: u32, sequence_index: usize) -> Option<(usize, usize)> {
        self.encoding.word_to_tokens(word_index, sequence_index)
    }

    pub fn word_to_chars(&self, word_index: u32, sequence_index: usize) -> Option<(usize, usize)> {
        self.encoding.word_to_chars(word_index, sequence_index)
    }

    pub fn token_to_sequence(&self, token_index: usize) -> Option<usize> {
        self.encoding.token_to_sequence(token_index)
    }

    pub fn token_to_chars(&self, token_index: usize) -> Option<(usize, usize)> {
        let (_, offsets) = self.encoding.token_to_chars(token_index)?;
        Some(offsets)
    }

    pub fn token_to_word(&self, token_index: usize) -> Option<u32> {
        let (_, word_idx) = self.encoding.token_to_word(token_index)?;
        Some(word_idx)
    }

    pub fn char_to_token(&self, char_pos: usize, sequence_index: usize) -> Option<usize> {
        self.encoding.char_to_token(char_pos, sequence_index)
    }

    pub fn char_to_word(&self, char_pos: usize, sequence_index: usize) -> Option<u32> {
        self.encoding.char_to_word(char_pos, sequence_index)
    }
}
