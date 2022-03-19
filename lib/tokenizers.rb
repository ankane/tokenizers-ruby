# extlib
require "tokenizers/ext"

# modules
require "tokenizers/char_bpe_tokenizer"
require "tokenizers/version"

module Tokenizers
  class Error < StandardError; end

  def self.from_pretrained(identifier, revision: "main", auth_token: nil)
    _from_pretrained(identifier, revision, auth_token)
  end
end
