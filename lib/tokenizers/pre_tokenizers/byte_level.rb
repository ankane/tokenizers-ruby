module Tokenizers
  module PreTokenizers
    class ByteLevel
      def self.new(add_prefix_space: true, use_regex: true)
        _new(add_prefix_space, use_regex)
      end
    end
  end
end
