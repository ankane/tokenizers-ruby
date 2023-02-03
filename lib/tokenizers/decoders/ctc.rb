module Tokenizers
  module Decoders
    class CTC
      def self.new(pad_token: "<pad>", word_delimiter_token: "|", cleanup: true)
        _new(pad_token, word_delimiter_token, cleanup)
      end
    end
  end
end
