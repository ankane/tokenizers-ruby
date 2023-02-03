module Tokenizers
  module Models
    class WordLevel
      def self.new(vocab: nil, unk_token: nil)
        _new(vocab, unk_token)
      end

      def self.from_file(vocab, unk_token: nil)
        _from_file(vocab, unk_token)
      end
    end
  end
end
