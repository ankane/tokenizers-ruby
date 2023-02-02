module Tokenizers
  class WordPieceDecoder
    def self.new(prefix: '##', cleanup: true)
      _new(prefix, cleanup)
    end
  end
end
