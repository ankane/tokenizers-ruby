module Tokenizers
  module Decoders
    class WordPiece
      def self.new(prefix: '##', cleanup: true)
        _new(prefix, cleanup)
      end
    end
  end
end
