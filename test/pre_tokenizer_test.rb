require_relative "test_helper"

class PreTokenizerTest < Minitest::Test
  def test_byte_level
    pre_tokenizer = Tokenizers::PreTokenizers::ByteLevel.new
    assert_instance_of Tokenizers::PreTokenizers::ByteLevel, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizers::PreTokenizer, pre_tokenizer

    pre_tokenizer = Tokenizers::PreTokenizers::ByteLevel.new(add_prefix_space: false, use_regex: false)

    assert_equal false, pre_tokenizer.add_prefix_space
    pre_tokenizer.add_prefix_space = true
    assert_equal true, pre_tokenizer.add_prefix_space

    assert_equal false, pre_tokenizer.use_regex
    pre_tokenizer.use_regex = true
    assert_equal true, pre_tokenizer.use_regex

    alphabet = Tokenizers::PreTokenizers::ByteLevel.alphabet
    assert 256, alphabet.size
  end

  def test_char_delimiter_split
    pre_tokenizer = Tokenizers::PreTokenizers::CharDelimiterSplit.new('a')
    assert_instance_of Tokenizers::PreTokenizers::CharDelimiterSplit, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizers::PreTokenizer, pre_tokenizer
 
    assert_equal 'a', pre_tokenizer.delimiter
    pre_tokenizer.delimiter = 'b'
    assert_equal 'b', pre_tokenizer.delimiter
  end

  def test_digits
    pre_tokenizer = Tokenizers::PreTokenizers::Digits.new
    assert_instance_of Tokenizers::PreTokenizers::Digits, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizers::PreTokenizer, pre_tokenizer

    pre_tokenizer = Tokenizers::PreTokenizers::Digits.new(individual_digits: true)

    assert_equal true, pre_tokenizer.individual_digits
    pre_tokenizer.individual_digits = false
    assert_equal false, pre_tokenizer.individual_digits
  end

  def test_metaspace
    pre_tokenizer = Tokenizers::PreTokenizers::Metaspace.new
    assert_instance_of Tokenizers::PreTokenizers::Metaspace, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizers::PreTokenizer, pre_tokenizer

    pre_tokenizer = Tokenizers::PreTokenizers::Metaspace.new(replacement: 'c', add_prefix_space: false)

    assert_equal 'c', pre_tokenizer.replacement
    pre_tokenizer.replacement = 'd'
    assert_equal 'd', pre_tokenizer.replacement

    assert_equal false, pre_tokenizer.add_prefix_space
    pre_tokenizer.add_prefix_space = true
    assert_equal true, pre_tokenizer.add_prefix_space
  end

  def test_punctuation
    pre_tokenizer = Tokenizers::PreTokenizers::Punctuation.new
    assert_instance_of Tokenizers::PreTokenizers::Punctuation, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizers::PreTokenizer, pre_tokenizer

    %w(removed isolated merged_with_previous merged_with_next contiguous).each do |b|
      Tokenizers::PreTokenizers::Punctuation.new(behavior: b)
    end

    assert_raises(ArgumentError) { Tokenizers::PreTokenizers::Punctuation.new(behavior: "invalid") }
  end

  def test_split
    pre_tokenizer = Tokenizers::PreTokenizers::Split.new("abc", "isolated")
    assert_instance_of Tokenizers::PreTokenizers::Split, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizers::PreTokenizer, pre_tokenizer

    %w(removed merged_with_previous merged_with_next contiguous).each do |b|
      Tokenizers::PreTokenizers::Split.new("abc", b)
    end

    Tokenizers::PreTokenizers::Split.new("abc", "isolated", invert: true)

    assert_raises(ArgumentError) { Tokenizers::PreTokenizers::Split.new("abc", "invalid") }
  end

  def test_whitespace
    pre_tokenizer = Tokenizers::PreTokenizers::Whitespace.new
    assert_instance_of Tokenizers::PreTokenizers::Whitespace, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizers::PreTokenizer, pre_tokenizer
  end

  def test_whitespace_split
    pre_tokenizer = Tokenizers::PreTokenizers::WhitespaceSplit.new
    assert_instance_of Tokenizers::PreTokenizers::WhitespaceSplit, pre_tokenizer
    assert_kind_of Tokenizers::PreTokenizers::PreTokenizer, pre_tokenizer
  end
end
