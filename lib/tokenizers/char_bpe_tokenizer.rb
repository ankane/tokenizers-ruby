module Tokenizers
  class CharBPETokenizer
    def initialize(vocab, merges)
      @tokenizer = Tokenizer.new(BPE.new(vocab, merges))
      @tokenizer.add_special_tokens(["<unk>"])
      @tokenizer.normalizer = BertNormalizer.new
      @tokenizer.pre_tokenizer = BertPreTokenizer.new
      @tokenizer.decoder = BPEDecoder.new
    end

    def encode(text)
      @tokenizer.encode(text)
    end

    def decode(ids)
      @tokenizer.decode(ids)
    end
  end
end
