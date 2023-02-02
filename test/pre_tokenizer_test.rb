require_relative "test_helper"

class PreTokenizerTest < Minitest::Test
  def test_byte_level
    pre_tokenizer = Tokenizers::ByteLevel.new
    assert_instance_of Tokenizers::ByteLevel, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer

    pre_tokenizer = Tokenizers::ByteLevel.new(add_prefix_space: false)
    assert_instance_of Tokenizers::ByteLevel, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer

    pre_tokenizer = Tokenizers::ByteLevel.new(use_regex: false)
    assert_instance_of Tokenizers::ByteLevel, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer

    alphabet = Tokenizers::ByteLevel.alphabet
    assert 256, alphabet.size
  end

  def test_char_delimiter_split
    pre_tokenizer = Tokenizers::CharDelimiterSplit.new('a')
    assert_instance_of Tokenizers::CharDelimiterSplit, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer
  end

  def test_digits
    pre_tokenizer = Tokenizers::Digits.new
    assert_instance_of Tokenizers::Digits, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer

    pre_tokenizer = Tokenizers::Digits.new(individual_digits: true)
    assert_instance_of Tokenizers::Digits, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer
  end

  def test_metaspace
    pre_tokenizer = Tokenizers::Metaspace.new
    assert_instance_of Tokenizers::Metaspace, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer

    pre_tokenizer = Tokenizers::Metaspace.new(replacement: 'c')
    assert_instance_of Tokenizers::Metaspace, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer

    pre_tokenizer = Tokenizers::Metaspace.new(add_prefix_space: false)
    assert_instance_of Tokenizers::Metaspace, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer
  end

  def test_punctuation
    pre_tokenizer = Tokenizers::Punctuation.new
    assert_instance_of Tokenizers::Punctuation, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer

    %w(removed isolated merged_with_previous merged_with_next contiguous).each do |b|
      pre_tokenizer = Tokenizers::Punctuation.new(behavior: b)
      assert_instance_of Tokenizers::Punctuation, pre_tokenizer
      assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer
    end

    assert_raises(ArgumentError) { Tokenizers::Punctuation.new(behavior: "invalid") }
  end

  def test_split
    pre_tokenizer = Tokenizers::Split.new("abc", "isolated")
    assert_instance_of Tokenizers::Split, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer

    %w(removed merged_with_previous merged_with_next contiguous).each do |b|
      pre_tokenizer = Tokenizers::Split.new("abc", b)
      assert_instance_of Tokenizers::Split, pre_tokenizer
      assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer
    end

    pre_tokenizer = Tokenizers::Split.new("abc", "isolated", invert: true)
    assert_instance_of Tokenizers::Split, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer

    assert_raises(ArgumentError) { Tokenizers::Split.new("abc", "invalid") }
  end

  def test_whitespace
    pre_tokenizer = Tokenizers::Whitespace.new
    assert_instance_of Tokenizers::Whitespace, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer
  end

  def test_whitespace_split
    pre_tokenizer = Tokenizers::WhitespaceSplit.new
    assert_instance_of Tokenizers::WhitespaceSplit, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizer, pre_tokenizer
  end
end
