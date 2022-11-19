require_relative "test_helper"

class TokenizersTest < Minitest::Test
  def test_from_pretrained
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")

    # encode
    encoded = tokenizer.encode("I can feel the magic, can you?")
    expected_ids = [146, 1169, 1631, 1103, 3974, 117, 1169, 1128, 136]
    expected_tokens = ["I", "can", "feel", "the", "magic", ",", "can", "you", "?"]
    assert_equal expected_ids, encoded.ids
    assert_equal expected_tokens, encoded.tokens

    # decode
    assert_equal "I can feel the magic, can you?", tokenizer.decode(encoded.ids)
  end

  def test_from_pretrained_bad_identifier
    # TODO use Tokenizers::Error
    error = assert_raises do
      Tokenizers.from_pretrained("bad")
    end
    assert_equal "Model \"bad\" on the Hub doesn't have a tokenizer", error.message
  end

  def test_char_bpe_tokenizer
    vocab = "test/support/roberta-base-vocab.json"
    merges = "test/support/roberta-base-merges.txt"
    tokenizer = Tokenizers::CharBPETokenizer.new(vocab, merges)

    # encode
    encoded = tokenizer.encode("I can feel the magic, can you?")
    expected_ids = [3, 3245, 3, 33763, 3, 212, 3, 119, 18879, 3, 3, 3245, 3, 9839, 3, 3]
    expected_tokens = ["<unk>", "ca", "<unk>", "fee", "<unk>", "th", "<unk>", "m", "agi", "<unk>", "<unk>", "ca", "<unk>", "yo", "<unk>", "<unk>"]
    assert_equal expected_ids, encoded.ids
    assert_equal expected_tokens, encoded.tokens

    # decode
    assert_equal "cafeethmagicayo", tokenizer.decode(encoded.ids)
  end
end
