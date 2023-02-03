module Tokenizers
  module Models
    class WordPiece
      def self.new(vocab: nil, **kwargs)
        _new(vocab, kwargs)
      end
    end
  end
end
