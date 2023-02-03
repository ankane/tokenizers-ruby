require_relative "test_helper"

class TrainerTest < Minitest::Test
  def test_bpe_trainer
    trainer = Tokenizers::Trainers::BpeTrainer.new
    assert_instance_of Tokenizers::Trainers::BpeTrainer, trainer
    assert_kind_of Tokenizers::Trainers::Trainer, trainer

    trainer = Tokenizers::Trainers::BpeTrainer.new(
      vocab_size: 10000,
      min_frequency: 1,
      show_progress: false,
      special_tokens: ["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"],
      limit_alphabet: 10000,
      initial_alphabet: ["a"],
      continuing_subword_prefix: "#x#",
      end_of_word_suffix: "~")

    assert_equal 10000, trainer.vocab_size
    trainer.vocab_size = 15000
    assert_equal 15000, trainer.vocab_size

    assert_equal 1, trainer.min_frequency
    trainer.min_frequency = 2
    assert_equal 2, trainer.min_frequency

    assert_equal false, trainer.show_progress
    trainer.show_progress = true
    assert_equal true, trainer.show_progress

    assert_equal ["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"], trainer.special_tokens
    trainer.special_tokens = ["[UNK]", "[CLS]", "[SEP]"]
    assert_equal ["[UNK]", "[CLS]", "[SEP]"], trainer.special_tokens

    assert_equal 10000, trainer.limit_alphabet
    trainer.limit_alphabet = 15000
    assert_equal 15000, trainer.limit_alphabet

    assert_equal ["a"], trainer.initial_alphabet
    trainer.initial_alphabet = ["b"]
    assert_equal ["b"], trainer.initial_alphabet

    assert_equal "#x#", trainer.continuing_subword_prefix
    trainer.continuing_subword_prefix = "##"
    assert_equal "##", trainer.continuing_subword_prefix

    assert_equal "~", trainer.end_of_word_suffix
    trainer.end_of_word_suffix = "#x#"
    assert_equal "#x#", trainer.end_of_word_suffix
  end

  def test_unigram_trainer
    trainer = Tokenizers::Trainers::UnigramTrainer.new
    assert_instance_of Tokenizers::Trainers::UnigramTrainer, trainer
    assert_kind_of Tokenizers::Trainers::Trainer, trainer

    trainer = Tokenizers::Trainers::UnigramTrainer.new(
      vocab_size: 8000,
      show_progress: false,
      special_tokens: ["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"],
      initial_alphabet: ["a"],
      shrinking_factor: 0.6,
      unk_token: "[UNK]",
      max_piece_length: 32,
      n_sub_iterations: 3)

    assert_equal 8000, trainer.vocab_size
    trainer.vocab_size = 15000
    assert_equal 15000, trainer.vocab_size

    assert_equal false, trainer.show_progress
    trainer.show_progress = true
    assert_equal true, trainer.show_progress

    assert_equal ["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"], trainer.special_tokens
    trainer.special_tokens = ["[UNK]", "[CLS]", "[SEP]"]
    assert_equal ["[UNK]", "[CLS]", "[SEP]"], trainer.special_tokens

    assert_equal ["a"], trainer.initial_alphabet
    trainer.initial_alphabet = ["b"]
    assert_equal ["b"], trainer.initial_alphabet
  end

  def test_word_level_trainer
    trainer = Tokenizers::Trainers::WordLevelTrainer.new
    assert_instance_of Tokenizers::Trainers::WordLevelTrainer, trainer
    assert_kind_of Tokenizers::Trainers::Trainer, trainer

    trainer = Tokenizers::Trainers::WordLevelTrainer.new(
      vocab_size: 10000,
      min_frequency: 1,
      show_progress: false,
      special_tokens: ["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"])

    assert_equal 10000, trainer.vocab_size
    trainer.vocab_size = 15000
    assert_equal 15000, trainer.vocab_size

    assert_equal 1, trainer.min_frequency
    trainer.min_frequency = 2
    assert_equal 2, trainer.min_frequency

    assert_equal false, trainer.show_progress
    trainer.show_progress = true
    assert_equal true, trainer.show_progress

    assert_equal ["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"], trainer.special_tokens
    trainer.special_tokens = ["[UNK]", "[CLS]", "[SEP]"]
    assert_equal ["[UNK]", "[CLS]", "[SEP]"], trainer.special_tokens
  end

  def test_word_piece_trainer
    trainer = Tokenizers::Trainers::WordPieceTrainer.new
    assert_instance_of Tokenizers::Trainers::WordPieceTrainer, trainer
    assert_kind_of Tokenizers::Trainers::Trainer, trainer

    trainer = Tokenizers::Trainers::WordPieceTrainer.new(
      vocab_size: 10000,
      min_frequency: 1,
      show_progress: false,
      special_tokens: ["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"],
      limit_alphabet: 10000,
      initial_alphabet: ["a"],
      continuing_subword_prefix: "#x#",
      end_of_word_suffix: "~")

    assert_equal 10000, trainer.vocab_size
    trainer.vocab_size = 15000
    assert_equal 15000, trainer.vocab_size

    assert_equal 1, trainer.min_frequency
    trainer.min_frequency = 2
    assert_equal 2, trainer.min_frequency

    assert_equal false, trainer.show_progress
    trainer.show_progress = true
    assert_equal true, trainer.show_progress

    assert_equal ["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"], trainer.special_tokens
    trainer.special_tokens = ["[UNK]", "[CLS]", "[SEP]"]
    assert_equal ["[UNK]", "[CLS]", "[SEP]"], trainer.special_tokens

    assert_equal 10000, trainer.limit_alphabet
    trainer.limit_alphabet = 15000
    assert_equal 15000, trainer.limit_alphabet

    assert_equal ["a"], trainer.initial_alphabet
    trainer.initial_alphabet = ["b"]
    assert_equal ["b"], trainer.initial_alphabet

    assert_equal "#x#", trainer.continuing_subword_prefix
    trainer.continuing_subword_prefix = "##"
    assert_equal "##", trainer.continuing_subword_prefix

    assert_equal "~", trainer.end_of_word_suffix
    trainer.end_of_word_suffix = "#x#"
    assert_equal "#x#", trainer.end_of_word_suffix
  end
end
