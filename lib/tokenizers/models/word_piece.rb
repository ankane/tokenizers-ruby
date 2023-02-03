module Tokenizers
  module Models
    class WordPiece
      def self.new(vocab: nil, unk_token: nil, max_input_chars_per_word: nil, continuing_subword_prefix: nil)
        kwargs = {}
        kwargs[:unk_token] = unk_token if unk_token
        kwargs[:max_input_chars_per_word] = max_input_chars_per_word if max_input_chars_per_word
        kwargs[:continuing_subword_prefix] = continuing_subword_prefix if continuing_subword_prefix
        _new(vocab, kwargs)
      end
    end
  end
end
