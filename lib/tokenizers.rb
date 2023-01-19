# ext
begin
  require "tokenizers/#{RUBY_VERSION.to_f}/tokenizers"
rescue LoadError
  require "tokenizers/tokenizers"
end

# modules
require "tokenizers/char_bpe_tokenizer"
require "tokenizers/encoding"
require "tokenizers/from_pretrained"
require "tokenizers/tokenizer"
require "tokenizers/version"

module Tokenizers
  class Error < StandardError; end

  extend FromPretrained
end
