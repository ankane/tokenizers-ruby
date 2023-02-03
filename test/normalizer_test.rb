require_relative "test_helper"

class NormalizerTest < Minitest::Test
  def test_bert_normalizer
    normalizer = Tokenizers::BertNormalizer.new
    assert_instance_of Tokenizers::BertNormalizer, normalizer
    assert_kind_of Tokenizers::Normalizer, normalizer

    normalizer = Tokenizers::BertNormalizer.new(clean_text: false)
    assert_instance_of Tokenizers::BertNormalizer, normalizer
    assert_kind_of Tokenizers::Normalizer, normalizer

    normalizer = Tokenizers::BertNormalizer.new(handle_chinese_chars: false)
    assert_instance_of Tokenizers::BertNormalizer, normalizer
    assert_kind_of Tokenizers::Normalizer, normalizer

    normalizer = Tokenizers::BertNormalizer.new(strip_accents: false)
    assert_instance_of Tokenizers::BertNormalizer, normalizer
    assert_kind_of Tokenizers::Normalizer, normalizer

    normalizer = Tokenizers::BertNormalizer.new(lowercase: false)
    assert_instance_of Tokenizers::BertNormalizer, normalizer
    assert_kind_of Tokenizers::Normalizer, normalizer
  end

  def test_lowercase
    normalizer = Tokenizers::Lowercase.new
    assert_instance_of Tokenizers::Lowercase, normalizer
    assert_kind_of Tokenizers::Lowercase, normalizer
  end

  def test_nfc
    normalizer = Tokenizers::NFC.new
    assert_instance_of Tokenizers::NFC, normalizer
    assert_kind_of Tokenizers::NFC, normalizer
  end

  def test_nfd
    normalizer = Tokenizers::NFD.new
    assert_instance_of Tokenizers::NFD, normalizer
    assert_kind_of Tokenizers::NFD, normalizer
  end

  def test_nfkc
    normalizer = Tokenizers::NFKC.new
    assert_instance_of Tokenizers::NFKC, normalizer
    assert_kind_of Tokenizers::NFKC, normalizer
  end

  def test_nfkd
    normalizer = Tokenizers::NFKD.new
    assert_instance_of Tokenizers::NFKD, normalizer
    assert_kind_of Tokenizers::NFKD, normalizer
  end

  def test_nmt
    normalizer = Tokenizers::Nmt.new
    assert_instance_of Tokenizers::Nmt, normalizer
    assert_kind_of Tokenizers::Nmt, normalizer
  end

  def test_replace
    normalizer = Tokenizers::Replace.new('abc', 'xyz')
    assert_instance_of Tokenizers::Replace, normalizer
    assert_kind_of Tokenizers::Replace, normalizer
  end

  def test_strip
    normalizer = Tokenizers::Strip.new
    assert_instance_of Tokenizers::Strip, normalizer
    assert_kind_of Tokenizers::Strip, normalizer

    normalizer = Tokenizers::Strip.new(left: false)
    assert_instance_of Tokenizers::Strip, normalizer
    assert_kind_of Tokenizers::Strip, normalizer

    normalizer = Tokenizers::Strip.new(right: false)
    assert_instance_of Tokenizers::Strip, normalizer
    assert_kind_of Tokenizers::Strip, normalizer
  end

  def test_strip_accents
    normalizer = Tokenizers::StripAccents.new
    assert_instance_of Tokenizers::StripAccents, normalizer
    assert_kind_of Tokenizers::StripAccents, normalizer
  end
end
