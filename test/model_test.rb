require_relative "test_helper"

class ModelTest < Minitest::Test
  def test_bpe
    model = Tokenizers::BPE.new
    assert_instance_of Tokenizers::BPE, model
    assert_kind_of Tokenizers::Model, model
  end
end
