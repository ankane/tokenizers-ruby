module Tokenizers
  module Processors
    class RobertaProcessing
      def self.new(sep, cls, trim_offsets: true, add_prefix_space: true)
        _new(sep, cls, trim_offsets, add_prefix_space)
      end
    end
  end
end
