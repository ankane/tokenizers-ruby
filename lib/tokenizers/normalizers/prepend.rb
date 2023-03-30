module Tokenizers
  module Normalizers
    class Prepend
      def self.new(prepend: "▁")
        _new(prepend)
      end
    end
  end
end
