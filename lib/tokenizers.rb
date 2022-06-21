# extlib

# Attempts to load the cross compiled gem first, falling back the native.
begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "#{$1}/tokenizers/tokenizers_ext"
rescue LoadError
  require "tokenizers/tokenizers_ext"
end

# modules
require "tokenizers/char_bpe_tokenizer"
require "tokenizers/version"

module Tokenizers
  class Error < StandardError; end

  def self.from_pretrained(identifier, revision: "main", auth_token: nil)
    _from_pretrained(identifier, revision, auth_token)
  end
end
