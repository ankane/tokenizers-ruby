require_relative "test_helper"

class ModelTest < Minitest::Test
  def test_bpe
    model = Tokenizers::Models::BPE.new
    assert_instance_of Tokenizers::Models::BPE, model
    assert_kind_of Tokenizers::Models::Model, model

    vocab = {"a" => 0, "b" => 1, "c" => 2, "d" => 3}
    model =
      Tokenizers::Models::BPE.new(
        vocab: vocab,
        merges: [],
        cache_capacity: 256,
        dropout: 0.5,
        unk_token: "[UNK]",
        continuing_subword_prefix: "##",
        end_of_word_suffix: "</end>",
        fuse_unk: true
      )
    assert_equal "[UNK]", model.unk_token
    model.unk_token = "[PAD]"
    assert_equal "[PAD]", model.unk_token

    assert_in_delta 0.5, model.dropout
    model.dropout = 0.6
    assert_in_delta 0.6, model.dropout

    assert_equal true, model.fuse_unk
    model.fuse_unk = false
    assert_equal false, model.fuse_unk
  end

  def test_word_level
    model = Tokenizers::Models::WordLevel.new
    assert_instance_of Tokenizers::Models::WordLevel, model
    assert_kind_of Tokenizers::Models::Model, model

    vocab = {"am" => 0}
    Tokenizers::Models::WordLevel.new(vocab: vocab, unk_token: "[UNK]")
  end

  def test_word_piece
    model = Tokenizers::Models::WordPiece.new
    assert_instance_of Tokenizers::Models::WordPiece, model
    assert_kind_of Tokenizers::Models::Model, model

    vocab = {"am" => 0}
    Tokenizers::Models::WordPiece.new(
      vocab: vocab,
      unk_token: "[UNK]",
      max_input_chars_per_word: 5,
      continuing_subword_prefix: "abc"
    )
  end

  def test_unigram
    model = Tokenizers::Models::Unigram.new
    assert_instance_of Tokenizers::Models::Unigram, model
    assert_kind_of Tokenizers::Models::Model, model

    Tokenizers::Models::Unigram.new(vocab: [["a", 0.117], ["b", 0.786]])
  end
end
