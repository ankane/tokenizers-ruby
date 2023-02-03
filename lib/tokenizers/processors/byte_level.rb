module Tokenizers
  module Processors
    class ByteLevel
      def self.new(trim_offsets: true)
        _new(trim_offsets)
      end
    end
  end
end
