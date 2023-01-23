module Tokenizers
  class CharBPETokenizer
    def initialize(vocab, merges, unk_token: "<unk>", suffix: "</w>")
      @tokenizer =
        Tokenizer.new(
          BPE._from_file(vocab, merges, {unk_token: unk_token, end_of_word_suffix: suffix})
        )
      @tokenizer.add_special_tokens([unk_token])
      @tokenizer.normalizer = BertNormalizer.new
      @tokenizer.pre_tokenizer = BertPreTokenizer.new
      @tokenizer.decoder = BPEDecoder.new
    end

    def encode(text, **options)
      @tokenizer.encode(text, **options)
    end

    def decode(ids)
      @tokenizer.decode(ids)
    end
  end
end
