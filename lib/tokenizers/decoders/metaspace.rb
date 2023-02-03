module Tokenizers
  module Decoders
    class Metaspace
      def self.new(replacement: "\u2581", add_prefix_space: true)
        _new(replacement, add_prefix_space)
      end
    end
  end
end
