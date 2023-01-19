module Tokenizers
  class Encoding
    def word_to_tokens(word_index, sequence_index = 0)
      _word_to_tokens(word_index, sequence_index)
    end

    def word_to_chars(word_index, sequence_index = 0)
      _word_to_chars(word_index, sequence_index)
    end

    def char_to_token(char_pos, sequence_index = 0)
      _char_to_token(char_pos, sequence_index)
    end

    def char_to_word(char_pos, sequence_index = 0)
      _char_to_word(word_index, sequence_index)
    end
  end
end