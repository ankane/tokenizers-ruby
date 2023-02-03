module Tokenizers
  module Normalizers
    class Strip
      def self.new(left: true, right: true)
        _new(left, right)
      end
    end
  end
end
