require_relative "test_helper"

class TrainerTest < Minitest::Test
  def test_bpe_trainer
    trainer = Tokenizers::BpeTrainer.new
    assert_instance_of Tokenizers::BpeTrainer, trainer
    assert_kind_of Tokenizers::BpeTrainer, trainer
  end
end
