module Tokenizers
  module Models
    class BPE
      def self.new(vocab: nil, merges: nil, **kwargs)
        _new(vocab, merges, kwargs)
      end
    end
  end
end
