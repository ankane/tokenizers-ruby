require_relative "test_helper"

class AddedTokenTest < Minitest::Test
  def test_content
    token = Tokenizers::AddedToken.new("test")
    assert_equal "test", token.content
    assert_equal false, token.rstrip
    assert_equal false, token.lstrip
    assert_equal false, token.single_word
    assert_equal true, token.normalized
    assert_equal false, token.special
  end
end
