module Tokenizers
  class Tokenizer
    def encode(sequence, pair = nil, add_special_tokens: true)
      _encode(sequence, pair, add_special_tokens)
    end
  end
end
