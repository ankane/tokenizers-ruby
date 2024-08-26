require_relative "test_helper"

class TokenizerTest < Minitest::Test
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

  def test_pretokenized_encoding
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")
    sequence = "A mellifluous sequence"
    pair = "And its malodorous pair"

    pretokenized_sequence = sequence.split(" ")
    pretokenized_pair = pair.split(" ")

    encoded_wout_pretokenization = tokenizer.encode(sequence, pair)
    encoded_with_pretokenization = tokenizer.encode(pretokenized_sequence, pretokenized_pair, is_pretokenized: true)

    assert_equal encoded_wout_pretokenization.tokens, encoded_with_pretokenization.tokens
  end

  def test_decode_with_special_tokens
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")

    token_ids = [101, 146, 1169, 1631, 1103, 3974, 117, 1169, 1128, 136, 102]

    assert_equal "[CLS] I can feel the magic, can you? [SEP]", tokenizer.decode(token_ids, skip_special_tokens: false)
  end

  def test_decode_batch
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")

    string_1 = "I can feel the magic, can you?"
    token_ids_1 = [101, 146, 1169, 1631, 1103, 3974, 117, 1169, 1128, 136, 102]

    string_2 = "Am I allowed to pass two text arguments?"
    token_ids_2 = [101, 7277, 146, 2148, 1106, 2789, 1160, 3087, 9989, 136, 102]

    assert_equal [string_1, string_2], tokenizer.decode_batch([token_ids_1, token_ids_2])

    assert_equal ["[CLS] #{string_1} [SEP]", "[CLS] #{string_2} [SEP]"], tokenizer.decode_batch([token_ids_1, token_ids_2], skip_special_tokens: false)
  end

  def test_vocab_size
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")

    size_without_added_tokens = tokenizer.vocab_size(with_added_tokens: false)
    assert_equal 28996, size_without_added_tokens

    tokenizer.add_tokens(["mellifluous", "malodorous"])
    size_with_added_tokens = tokenizer.vocab_size
    assert_equal 28998, size_with_added_tokens
  end

  def test_vocab
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")

    vocab_without_added_tokens = tokenizer.vocab(with_added_tokens: false)
    assert_equal 28996, vocab_without_added_tokens.size
    assert_equal 15011, vocab_without_added_tokens["upstream"]
    assert_nil vocab_without_added_tokens["mellifluous"]

    tokenizer.add_tokens(["mellifluous", "malodorous"])
    vocab_with_added_tokens = tokenizer.vocab
    assert_equal 28998, vocab_with_added_tokens.size
    assert_equal 15011, vocab_with_added_tokens["upstream"]
    assert_equal 28996, vocab_with_added_tokens["mellifluous"]
  end

  def test_padding
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")
    assert_nil tokenizer.padding

    tokenizer.enable_padding
    default_padding = {"length"=>nil, "pad_id"=>0, "pad_type_id"=>0, "pad_token"=>"[PAD]", "pad_to_multiple_of"=>nil, "direction"=>"right"}
    assert_equal default_padding, tokenizer.padding

    tokenizer.enable_padding(length: 1024, direction: "left", pad_to_multiple_of: 256, pad_id: 29000, pad_type_id: 1, pad_token: "[SSS]")
    configured_padding = {"length"=>1024, "pad_id"=>29000, "pad_type_id"=>1, "pad_token"=>"[SSS]", "pad_to_multiple_of"=>256, "direction"=>"left"}
    assert_equal configured_padding, tokenizer.padding

    tokenizer.no_padding
    assert_nil tokenizer.padding
  end

  def test_truncation
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")
    assert_nil tokenizer.truncation

    tokenizer.enable_truncation(1024)
    default_params_with_length = {"max_length"=>1024, "stride"=>0, "strategy"=>"longest_first", "direction"=>"right"}
    assert_equal default_params_with_length, tokenizer.truncation

    tokenizer.enable_truncation(2048, stride: 20, direction: "left", strategy: "only_first")
    custom_params_with_length = {"max_length"=>2048, "stride"=>20, "strategy"=>"only_first", "direction"=>"left"}
    assert_equal custom_params_with_length, tokenizer.truncation

    tokenizer.no_truncation
    assert_nil tokenizer.truncation
  end

  def test_serialization
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")
    assert_nil tokenizer.vocab["mellifluous"]

    tokenizer.add_tokens(["mellifluous", "malodorous"])
    preserialization_size_with_added_tokens = tokenizer.vocab_size
    assert_equal 28996, tokenizer.vocab["mellifluous"]

    as_pretty_str = tokenizer.to_s(pretty: true)
    assert_equal 29163, as_pretty_str.count("\n")
    pretty_path = "/tmp/pretty-tokenizer.json"
    tokenizer.save(pretty_path, pretty: true)

    # Compare file content
    pretty_from_file = File.read(pretty_path)
    assert_equal as_pretty_str, pretty_from_file

    new_tokenizer = Tokenizers.from_file(pretty_path)
    assert_equal preserialization_size_with_added_tokens, new_tokenizer.vocab_size
    assert_equal 28996, new_tokenizer.vocab["mellifluous"]
  end

  def test_num_special_tokens_to_add
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")
    assert_equal 3, tokenizer.num_special_tokens_to_add(true)
    assert_equal 2, tokenizer.num_special_tokens_to_add(false)
  end

  def test_decoder
    tokenizer = Tokenizers.from_pretrained("bert-base-cased")
    assert_kind_of Tokenizers::Decoders::WordPiece, tokenizer.decoder
  end
end
