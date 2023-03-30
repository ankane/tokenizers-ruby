module Tokenizers
  module Decoders
    class Strip
      def self.new(content: " ", start: 0, stop: 0)
        _new(content, start, stop)
      end
    end
  end
end
