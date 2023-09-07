module Tokenizers
  module Models
    class Unigram
      def self.new(vocab: nil, unk_id: nil, byte_fallback: nil)
        _new(vocab, unk_id, byte_fallback)
      end
    end
  end
end
