require_relative "test_helper"

class NormalizerTest < Minitest::Test
  def test_bert_normalizer
    model = Tokenizers::BertNormalizer.new
    assert_instance_of Tokenizers::BertNormalizer, model
    assert_kind_of Tokenizers::Normalizer, model
  end
end
