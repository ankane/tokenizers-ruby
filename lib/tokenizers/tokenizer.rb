module Tokenizers
  class Tokenizer
    extend FromPretrained

    def encode(sequence, pair = nil, is_pretokenized: false, add_special_tokens: true)
      _encode(sequence, pair, is_pretokenized, add_special_tokens)
    end

    def encode_batch(input, is_pretokenized: false, add_special_tokens: true)
      _encode_batch(input, is_pretokenized, add_special_tokens)
    end

    def enable_padding(**options)
      _enable_padding(options)
    end
  end
end
