require_relative "test_helper"

class TrainerTest < Minitest::Test
  def test_bpe_trainer
    trainer = Tokenizers::Trainers::BpeTrainer.new
    assert_instance_of Tokenizers::Trainers::BpeTrainer, trainer
    assert_kind_of Tokenizers::Trainers::BpeTrainer, trainer
  end
end
