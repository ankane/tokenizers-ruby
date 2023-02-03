require_relative "test_helper"

class QuicktourTest < Minitest::Test
  def setup
    skip unless data_path
  end

  # https://huggingface.co/docs/tokenizers/quicktour
  def test_works
    tokenizer = Tokenizers::Tokenizer.new(Tokenizers::Models::BPE.new(unk_token: "[UNK]"))

    trainer = Tokenizers::Trainers::BpeTrainer.new(special_tokens: ["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"])

    tokenizer.pre_tokenizer = Tokenizers::PreTokenizers::Whitespace.new

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

    tokenizer.post_processor = Tokenizers::Processors::TemplateProcessing.new(
      single: "[CLS] $A [SEP]",
      pair: "[CLS] $A [SEP] $B:1 [SEP]:1",
      special_tokens: [
        ["[CLS]", tokenizer.token_to_id("[CLS]")],
        ["[SEP]", tokenizer.token_to_id("[SEP]")]
      ]
    )

    output = tokenizer.encode("Hello, y'all! How are you 😁 ?")
    assert_equal ["[CLS]", "Hello", ",", "y", "'", "all", "!", "How", "are", "you", "[UNK]", "?", "[SEP]"], output.tokens

    output = tokenizer.encode("Hello, y'all!", "How are you 😁 ?")
    assert_equal ["[CLS]", "Hello", ",", "y", "'", "all", "!", "[SEP]", "How", "are", "you", "[UNK]", "?", "[SEP]"], output.tokens

    assert_equal [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1], output.type_ids

    output = tokenizer.encode_batch(["Hello, y'all!", "How are you 😁 ?"])

    output =
      tokenizer.encode_batch(
        [["Hello, y'all!", "How are you 😁 ?"], ["Hello to you too!", "I'm fine, thank you!"]]
      )

    tokenizer.enable_padding(pad_id: 3, pad_token: "[PAD]")

    output = tokenizer.encode_batch(["Hello, y'all!", "How are you 😁 ?"])
    assert_equal ["[CLS]", "How", "are", "you", "[UNK]", "?", "[SEP]", "[PAD]"], output[1].tokens

    assert_equal [1, 1, 1, 1, 1, 1, 1, 0], output[1].attention_mask
  end

  private

  def data_path
    ENV["DATA_PATH"]
  end
end
