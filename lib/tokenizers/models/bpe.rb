module Tokenizers
  module Models
    class BPE
      def self.new(vocab: nil, merges: nil, cache_capacity: nil, dropout: nil, unk_token: nil,
                   continuing_subword_prefix: nil, end_of_word_suffix: nil, fuse_unk: nil)
        kwargs = {}
        kwargs[:cache_capacity] = cache_capacity if cache_capacity
        kwargs[:dropout] = dropout if dropout
        kwargs[:unk_token] = unk_token if unk_token
        kwargs[:continuing_subword_prefix] = continuing_subword_prefix if continuing_subword_prefix
        kwargs[:end_of_word_suffix] = end_of_word_suffix if end_of_word_suffix
        kwargs[:fuse_unk] = fuse_unk if fuse_unk
        _new(vocab, merges, kwargs)
      end
    end
  end
end
