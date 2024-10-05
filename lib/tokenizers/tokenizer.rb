module Tokenizers
  class Tokenizer
    extend FromPretrained

    def save(path, pretty: false)
      _save(path, pretty)
    end

    def encode(sequence, pair = nil, is_pretokenized: false, add_special_tokens: true)
      _encode(sequence, pair, is_pretokenized, add_special_tokens)
    end

    def encode_batch(input, is_pretokenized: false, add_special_tokens: true)
      _encode_batch(input, is_pretokenized, add_special_tokens)
    end

    def decode(ids, skip_special_tokens: true)
      _decode(ids, skip_special_tokens)
    end

    def decode_batch(sequences, skip_special_tokens: true)
      _decode_batch(sequences, skip_special_tokens)
    end

    def enable_padding(**options)
      _enable_padding(options)
    end

    def enable_truncation(max_length, **options)
      _enable_truncation(max_length, options)
    end

    def vocab(with_added_tokens: true)
      _vocab(with_added_tokens)
    end

    def vocab_size(with_added_tokens: true)
      _vocab_size(with_added_tokens)
    end
  end
end
