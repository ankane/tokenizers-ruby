module Tokenizers
  module PreTokenizers
    class Split
      def self.new(pattern, behavior, invert: false)
        _new(pattern, behavior, invert)
      end
    end
  end
end
