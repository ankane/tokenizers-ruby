require_relative "test_helper"

class TrainerTest < Minitest::Test
  def test_bpe_trainer
    trainer = Tokenizers::Trainers::BpeTrainer.new
    assert_instance_of Tokenizers::Trainers::BpeTrainer, trainer
    assert_kind_of Tokenizers::Trainers::Trainer, trainer

    Tokenizers::Trainers::BpeTrainer.new(
      vocab_size: 10000,
      min_frequency: 1,
      show_progress: false,
      special_tokens: ["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"],
      limit_alphabet: 10000,
      initial_alphabet: ["a"],
      continuing_subword_prefix: "#x#",
      end_of_word_suffix: "~")
  end

  def test_unigram_trainer
    trainer = Tokenizers::Trainers::UnigramTrainer.new
    assert_instance_of Tokenizers::Trainers::UnigramTrainer, trainer
    assert_kind_of Tokenizers::Trainers::Trainer, trainer

    Tokenizers::Trainers::UnigramTrainer.new(
      vocab_size: 8000,
      show_progress: false,
      special_tokens: ["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"],
      shrinking_factor: 0.6,
      unk_token: "[UNK]",
      max_piece_length: 32,
      n_sub_iterations: 3)
  end

  def test_word_level_trainer
    trainer = Tokenizers::Trainers::WordLevelTrainer.new
    assert_instance_of Tokenizers::Trainers::WordLevelTrainer, trainer
    assert_kind_of Tokenizers::Trainers::Trainer, trainer

    Tokenizers::Trainers::WordLevelTrainer.new(
      vocab_size: 10000,
      min_frequency: 1,
      show_progress: false,
      special_tokens: ["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"])
  end

  def test_word_piece_trainer
    trainer = Tokenizers::Trainers::WordPieceTrainer.new
    assert_instance_of Tokenizers::Trainers::WordPieceTrainer, trainer
    assert_kind_of Tokenizers::Trainers::Trainer, trainer

    Tokenizers::Trainers::WordPieceTrainer.new(
      vocab_size: 10000,
      min_frequency: 1,
      show_progress: false,
      special_tokens: ["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"],
      limit_alphabet: 10000,
      initial_alphabet: ["a"],
      continuing_subword_prefix: "#x#",
      end_of_word_suffix: "~")
  end
end
