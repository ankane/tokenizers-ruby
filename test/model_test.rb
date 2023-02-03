require_relative "test_helper"

class ModelTest < Minitest::Test
  def test_bpe
    model = Tokenizers::Models::BPE.new
    assert_instance_of Tokenizers::Models::BPE, model
    assert_kind_of Tokenizers::Models::Model, model
  end
end
