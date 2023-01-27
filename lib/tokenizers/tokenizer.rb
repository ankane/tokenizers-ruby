module Tokenizers
  class Tokenizer
    extend FromPretrained

    def encode(sequence, pair = nil, add_special_tokens: true)
      _encode(sequence, pair, add_special_tokens)
    end

    def encode_batch(input, add_special_tokens: true)
      _encode_batch(input, false, add_special_tokens)
    end

    def enable_padding(**options)
      _enable_padding(options)
    end
  end
end
