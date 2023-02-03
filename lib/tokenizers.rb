# ext
begin
  require_relative "tokenizers/#{RUBY_VERSION.to_f}/tokenizers"
rescue LoadError
  require_relative "tokenizers/tokenizers"
end

# decoders
require_relative "tokenizers/decoders/bpe_decoder"
require_relative "tokenizers/decoders/ctc"
require_relative "tokenizers/decoders/metaspace"
require_relative "tokenizers/decoders/word_piece"

# models
require_relative "tokenizers/models/bpe"
require_relative "tokenizers/models/word_level"
require_relative "tokenizers/models/word_piece"
require_relative "tokenizers/models/unigram"

# normalizers
require_relative "tokenizers/normalizers/bert_normalizer"
require_relative "tokenizers/normalizers/strip"

# pre-tokenizers
require_relative "tokenizers/pre_tokenizers/byte_level"
require_relative "tokenizers/pre_tokenizers/digits"
require_relative "tokenizers/pre_tokenizers/metaspace"
require_relative "tokenizers/pre_tokenizers/punctuation"
require_relative "tokenizers/pre_tokenizers/split"

# processors
require_relative "tokenizers/processors/byte_level"
require_relative "tokenizers/processors/roberta_processing"
require_relative "tokenizers/processors/template_processing"

# trainers
require_relative "tokenizers/trainers/bpe_trainer"
require_relative "tokenizers/trainers/unigram_trainer"
require_relative "tokenizers/trainers/word_level_trainer"
require_relative "tokenizers/trainers/word_piece_trainer"

# other
require_relative "tokenizers/char_bpe_tokenizer"
require_relative "tokenizers/encoding"
require_relative "tokenizers/from_pretrained"
require_relative "tokenizers/tokenizer"
require_relative "tokenizers/version"

module Tokenizers
  class Error < StandardError; end

  def self.from_pretrained(...)
    Tokenizer.from_pretrained(...)
  end

  def self.from_file(...)
    Tokenizer.from_file(...)
  end
end
