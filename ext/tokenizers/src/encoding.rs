use magnus::{method, Module, RArray, RModule, Ruby};
use tk::{Encoding, Offsets};

use super::RbResult;

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
    pub fn get_n_sequences(&self) -> usize {
        self.encoding.n_sequences()
    }

    pub fn get_ids(&self) -> Vec<u32> {
        self.encoding.get_ids().to_vec()
    }

    pub fn get_tokens(&self) -> Vec<String> {
        self.encoding.get_tokens().to_vec()
    }

    pub fn get_word_ids(&self) -> Vec<Option<u32>> {
        self.encoding.get_word_ids().to_vec()
    }

    pub fn get_sequence_ids(&self) -> Vec<Option<usize>> {
        self.encoding.get_sequence_ids()
    }

    pub fn get_type_ids(&self) -> Vec<u32> {
        self.encoding.get_type_ids().to_vec()
    }

    pub fn get_offsets(&self) -> Vec<(usize, usize)> {
        self.encoding.get_offsets().to_vec()
    }

    pub fn get_special_tokens_mask(&self) -> Vec<u32> {
        self.encoding.get_special_tokens_mask().to_vec()
    }

    pub fn get_attention_mask(&self) -> Vec<u32> {
        self.encoding.get_attention_mask().to_vec()
    }

    pub fn get_overflowing(ruby: &Ruby, rb_self: &Self) -> RArray {
        ruby.ary_from_iter(
            rb_self
                .encoding
                .get_overflowing()
                .clone()
                .into_iter()
                .map(Into::<RbEncoding>::into),
        )
    }

    pub fn word_to_tokens(&self, word_index: u32, sequence_index: usize) -> Option<(usize, usize)> {
        self.encoding.word_to_tokens(word_index, sequence_index)
    }

    pub fn word_to_chars(&self, word_index: u32, sequence_index: usize) -> Option<Offsets> {
        self.encoding.word_to_chars(word_index, sequence_index)
    }

    pub fn token_to_sequence(&self, token_index: usize) -> Option<usize> {
        self.encoding.token_to_sequence(token_index)
    }

    pub fn token_to_chars(&self, token_index: usize) -> Option<Offsets> {
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

pub fn init_encoding(ruby: &Ruby, module: &RModule) -> RbResult<()> {
    let class = module.define_class("Encoding", ruby.class_object())?;
    class.define_method("n_sequences", method!(RbEncoding::get_n_sequences, 0))?;
    class.define_method("ids", method!(RbEncoding::get_ids, 0))?;
    class.define_method("tokens", method!(RbEncoding::get_tokens, 0))?;
    class.define_method("word_ids", method!(RbEncoding::get_word_ids, 0))?;
    class.define_method("sequence_ids", method!(RbEncoding::get_sequence_ids, 0))?;
    class.define_method("type_ids", method!(RbEncoding::get_type_ids, 0))?;
    class.define_method("offsets", method!(RbEncoding::get_offsets, 0))?;
    class.define_method(
        "special_tokens_mask",
        method!(RbEncoding::get_special_tokens_mask, 0),
    )?;
    class.define_method("attention_mask", method!(RbEncoding::get_attention_mask, 0))?;
    class.define_method("overflowing", method!(RbEncoding::get_overflowing, 0))?;
    class.define_method("_word_to_tokens", method!(RbEncoding::word_to_tokens, 2))?;
    class.define_method("_word_to_chars", method!(RbEncoding::word_to_chars, 2))?;
    class.define_method(
        "token_to_sequence",
        method!(RbEncoding::token_to_sequence, 1),
    )?;
    class.define_method("token_to_chars", method!(RbEncoding::token_to_chars, 1))?;
    class.define_method("token_to_word", method!(RbEncoding::token_to_word, 1))?;
    class.define_method("_char_to_token", method!(RbEncoding::char_to_token, 2))?;
    class.define_method("_char_to_word", method!(RbEncoding::char_to_word, 2))?;

    Ok(())
}
