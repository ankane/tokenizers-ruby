module Tokenizers
  module Processors
    class TemplateProcessing
      def self.new(single: nil, pair: nil, special_tokens: nil)
        _new(single, pair, special_tokens)
      end
    end
  end
end
