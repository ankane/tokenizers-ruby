require_relative "test_helper"

class TokenizersTest < Minitest::Test
  def test_from_pretrained_bert
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

  def test_from_pretrained_gpt2
    tokenizer = Tokenizers.from_pretrained("gpt2")

    # encode
    encoded = tokenizer.encode("Mythological creatures like the mighty gryphon inspire awe!")
    expected_ids = [41444, 2770, 8109, 588, 262, 18680, 308, 563, 746, 261, 18330, 25030, 0]
    expected_tokens = ["Myth", "ological", "Ġcreatures", "Ġlike", "Ġthe", "Ġmighty", "Ġg", "ry", "ph", "on", "Ġinspire", "Ġawe", "!"]
    expected_word_ids = [0, 0, 1, 2, 3, 4, 5, 5, 5, 5, 6, 7, 8]

    assert_equal expected_ids, encoded.ids
    assert_equal expected_tokens, encoded.tokens
    assert_equal expected_word_ids, encoded.word_ids

    # decode
    assert_equal "Mythological creatures like the mighty gryphon inspire awe!", tokenizer.decode(encoded.ids)
  end

  def test_from_pretrained_bad_identifier
    error = assert_raises(Tokenizers::Error) do
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
