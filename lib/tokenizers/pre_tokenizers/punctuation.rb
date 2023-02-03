module Tokenizers
  module PreTokenizers
    class Punctuation
      def self.new(behavior: "isolated")
        _new(behavior)
      end
    end
  end
end
