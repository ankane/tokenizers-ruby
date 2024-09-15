require_relative "test_helper"

class AddedTokenTest < Minitest::Test
  def test_content
    token = Tokenizers::AddedToken.new("test")
    assert_equal "test", token.content
  end
end
