require_relative "test_helper"

class TokenizersTest < Minitest::Test
  # https://huggingface.co/docs/tokenizers/quicktour
  def test_quicktour
    data_path = ENV["DATA_PATH"]

    skip unless data_path

    tokenizer = Tokenizers::Tokenizer.new(Tokenizers::BPE.new(unk_token: "[UNK]"))

    trainer = Tokenizers::BpeTrainer.new(special_tokens: ["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"])

    tokenizer.pre_tokenizer = Tokenizers::Whitespace.new

    files = ["test", "train", "valid"].map { |split| "#{data_path}/wikitext-103-raw/wiki.#{split}.raw" }
    tokenizer.train(files, trainer)

    tokenizer.save("/tmp/tokenizer-wiki.json")

    tokenizer = Tokenizers::Tokenizer.from_file("/tmp/tokenizer-wiki.json")

    output = tokenizer.encode("Hello, y'all! How are you 😁 ?")

    assert_equal ["Hello", ",", "y", "'", "all", "!", "How", "are", "you", "[UNK]", "?"], output.tokens

    assert_equal [27253, 16, 93, 11, 5097, 5, 7961, 5112, 6218, 0, 35], output.ids

    assert_equal [26, 27], output.offsets[9]

    sentence = "Hello, y'all! How are you 😁 ?"
    assert_equal "😁", sentence[26...27]

    assert_equal 2, tokenizer.token_to_id("[SEP]")
  end

  def test_from_pretrained_bert
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")

    # encode
    encoded = tokenizer.encode("I can feel the magic, can you?")
    expected_ids = [101, 146, 1169, 1631, 1103, 3974, 117, 1169, 1128, 136, 102]
    expected_tokens = ["[CLS]", "I", "can", "feel", "the", "magic", ",", "can", "you", "?", "[SEP]"]
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

    assert_equal [6, 10], encoded.word_to_tokens(5)
    assert_equal [38, 46], encoded.word_to_chars(5)
    assert_equal [4, 12], encoded.token_to_chars(1)
    assert_equal 0, encoded.token_to_word(1)
    assert_equal 7, encoded.char_to_token(41)
    assert_equal 10, encoded.char_to_token(48)

    # decode
    assert_equal "Mythological creatures like the mighty gryphon inspire awe!", tokenizer.decode(encoded.ids)
  end

  def test_from_pretrained_bad_identifier
    error = assert_raises(Tokenizers::Error) do
      Tokenizers.from_pretrained("bad")
    end
    assert_equal "Model \"bad\" on the Hub doesn't have a tokenizer", error.message
  end

  def test_add_special_tokens
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")

    # encode
    encoded = tokenizer.encode("I can feel the magic, can you?", add_special_tokens: false)
    expected_ids = [146, 1169, 1631, 1103, 3974, 117, 1169, 1128, 136]
    expected_tokens = ["I", "can", "feel", "the", "magic", ",", "can", "you", "?"]
    assert_equal expected_ids, encoded.ids
    assert_equal expected_tokens, encoded.tokens

    # decode
    assert_equal "I can feel the magic, can you?", tokenizer.decode(encoded.ids)
  end

  def test_char_bpe_tokenizer
    vocab = "test/support/roberta-base-vocab.json"
    merges = "test/support/roberta-base-merges.txt"
    tokenizer = Tokenizers::CharBPETokenizer.new(vocab, merges)

    # encode
    encoded = tokenizer.encode("I can feel the magic, can you?", add_special_tokens: false)
    expected_ids = [3, 3245, 3, 33763, 3, 212, 3, 119, 18879, 3, 3, 3245, 3, 9839, 3, 3]
    expected_tokens = ["<unk>", "ca", "<unk>", "fee", "<unk>", "th", "<unk>", "m", "agi", "<unk>", "<unk>", "ca", "<unk>", "yo", "<unk>", "<unk>"]
    assert_equal expected_ids, encoded.ids
    assert_equal expected_tokens, encoded.tokens

    # decode
    assert_equal "cafeethmagicayo", tokenizer.decode(encoded.ids)
  end

  def test_id_token_conversion
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")

    assert_equal 1169, tokenizer.token_to_id("can")
    assert_equal "magic", tokenizer.id_to_token(3974)
  end

  def test_multibyte_offsets
    tokenizer = Tokenizers.from_pretrained("gpt2")
    encoded = tokenizer.encode("I wanted to convert 10000 ¥ to $.")
    expected_tokens = ["I", "Ġwanted", "Ġto", "Ġconvert", "Ġ10000", "ĠÂ¥", "Ġto", "Ġ$", "."]
    expected_offsets = [[0, 1], [1, 8], [8, 11], [11, 19], [19, 25], [25, 27], [27, 30], [30, 32], [32, 33]]

    assert_equal expected_tokens, encoded.tokens
    assert_equal expected_offsets, encoded.offsets
  end

  def test_pair_encoding
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")
    question = "Am I allowed to pass two text arguments?"
    answer = "Yes I am!"
    encoded = tokenizer.encode(question, answer)

    expected_tokens = ["[CLS]", "Am", "I", "allowed", "to", "pass", "two", "text", "arguments", "?", "[SEP]", "Yes", "I", "am", "!", "[SEP]"]
    assert_equal expected_tokens, encoded.tokens
  end
end
