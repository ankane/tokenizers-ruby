# ext
begin
  require_relative "tokenizers/#{RUBY_VERSION.to_f}/tokenizers"
rescue LoadError
  require_relative "tokenizers/tokenizers"
end

# modules
require_relative "tokenizers/bpe"
require_relative "tokenizers/bpe_trainer"
require_relative "tokenizers/char_bpe_tokenizer"
require_relative "tokenizers/encoding"
require_relative "tokenizers/from_pretrained"
require_relative "tokenizers/tokenizer"
require_relative "tokenizers/version"

module Tokenizers
  class Error < StandardError; end

  extend FromPretrained
end
