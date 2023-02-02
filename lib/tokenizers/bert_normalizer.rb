module Tokenizers
  class BertNormalizer
    def self.new(clean_text: true, handle_chinese_chars: true, strip_accents: nil, lowercase: true)
      _new(clean_text, handle_chinese_chars, strip_accents, lowercase)
    end
  end
end
