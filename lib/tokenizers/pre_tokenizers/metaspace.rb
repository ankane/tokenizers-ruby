module Tokenizers
  module PreTokenizers
    class Metaspace
      def self.new(replacement: "\u2581", prepend_scheme: "always", split: true)
        _new(replacement, prepend_scheme, split)
      end
    end
  end
end
