module Tokenizers
  class Tokenizer
    # TODO change add_special_tokens default to true in 0.3.0
    def encode(sequence, add_special_tokens: false)
      _encode(sequence, add_special_tokens)
    end
  end
end
