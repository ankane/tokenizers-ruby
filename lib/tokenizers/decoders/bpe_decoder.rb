module Tokenizers
  module Decoders
    class BPEDecoder
      def self.new(suffix: "</w>")
        _new(suffix)
      end
    end
  end
end
