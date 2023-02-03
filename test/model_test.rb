require_relative "test_helper"

class ModelTest < Minitest::Test
  def test_bpe
    model = Tokenizers::Models::BPE.new
    assert_instance_of Tokenizers::Models::BPE, model
    assert_kind_of Tokenizers::Models::Model, model

    vocab = Hash.new
    vocab["a"] = 0
    vocab["b"] = 1
    vocab["c"] = 2
    vocab["d"] = 3
    model = Tokenizers::Models::BPE.new(vocab: vocab,
                                        merges: [],
                                        cache_capacity: 256,
                                        dropout: 0.5,
                                        unk_token: "[UNK]",
                                        continuing_subword_prefix: "##",
                                        end_of_word_suffix: "</end>",
                                        fuse_unk: true)
  end

  def test_word_level
    model = Tokenizers::Models::WordLevel.new
    assert_instance_of Tokenizers::Models::WordLevel, model
    assert_kind_of Tokenizers::Models::Model, model

    vocab = Hash.new
    vocab["am"] = 0
    model = Tokenizers::Models::WordLevel.new(vocab: vocab,
                                              unk_token: "[UNK]")
  end

  def test_word_piece
    model = Tokenizers::Models::WordPiece.new
    assert_instance_of Tokenizers::Models::WordPiece, model
    assert_kind_of Tokenizers::Models::Model, model

    vocab = Hash.new
    vocab["am"] = 0
    model = Tokenizers::Models::WordPiece.new(vocab: vocab,
                                              unk_token: "[UNK]",
                                              max_input_chars_per_word: 5,
                                              continuing_subword_prefix: "abc")
  end

  def test_unigram
    model = Tokenizers::Models::Unigram.new
    assert_instance_of Tokenizers::Models::Unigram, model
    assert_kind_of Tokenizers::Models::Model, model

    model = Tokenizers::Models::Unigram.new(vocab: [["a", 0.117], ["b", 0.786]])
  end
end
