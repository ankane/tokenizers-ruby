module Tokenizers
  class Tokenizer
    # TODO change add_special_tokens default to true in 0.3.0
    def encode(sequence, pair = nil, add_special_tokens: nil)
      if add_special_tokens.nil?
        warn "[tokenizers] add_special_tokens will default to true in 0.3.0. Pass add_special_tokens: true/false to silence this warning."
        add_special_tokens = false
      end
      _encode(sequence, pair, add_special_tokens)
    end
  end
end
