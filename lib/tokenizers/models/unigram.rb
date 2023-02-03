module Tokenizers
  module Models
    class Unigram
      def self.new(vocab: nil, unk_id: nil)
        _new(vocab, unk_id)
      end
    end
  end
end
