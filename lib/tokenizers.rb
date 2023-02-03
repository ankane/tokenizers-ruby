# ext
begin
  require_relative "tokenizers/#{RUBY_VERSION.to_f}/tokenizers"
rescue LoadError
  require_relative "tokenizers/tokenizers"
end

# modules
require_relative "tokenizers/bert_normalizer"
require_relative "tokenizers/bpe"
require_relative "tokenizers/bpe_decoder"
require_relative "tokenizers/bpe_trainer"
require_relative "tokenizers/byte_level"
require_relative "tokenizers/char_bpe_tokenizer"
require_relative "tokenizers/ctc"
require_relative "tokenizers/digits"
require_relative "tokenizers/encoding"
require_relative "tokenizers/from_pretrained"
require_relative "tokenizers/metaspace"
require_relative "tokenizers/metaspace_decoder"
require_relative "tokenizers/punctuation"
require_relative "tokenizers/split"
require_relative "tokenizers/strip"
require_relative "tokenizers/template_processing"
require_relative "tokenizers/tokenizer"
require_relative "tokenizers/version"
require_relative "tokenizers/word_piece_decoder"

module Tokenizers
  class Error < StandardError; end

  def self.from_pretrained(...)
    Tokenizer.from_pretrained(...)
  end

  def self.from_file(...)
    Tokenizer.from_file(...)
  end
end
