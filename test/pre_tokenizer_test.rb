require_relative "test_helper"

class PreTokenizerTest < Minitest::Test
  def test_whitespace
    pre_tokenizer = Tokenizers::Whitespace.new
    assert_instance_of Tokenizers::Whitespace, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer
  end
end
