require_relative "test_helper"

class NormalizerTest < Minitest::Test
  def test_normalize_str
    normalizer = Tokenizers::Normalizers::Sequence.new([Tokenizers::Normalizers::NFD.new, Tokenizers::Normalizers::StripAccents.new])
    assert_equal "Hello how are u?", normalizer.normalize_str("Héllò hôw are ü?")
  end

  def test_bert_normalizer
    normalizer = Tokenizers::Normalizers::BertNormalizer.new
    assert_instance_of Tokenizers::Normalizers::BertNormalizer, normalizer
    assert_kind_of Tokenizers::Normalizers::Normalizer, normalizer

    normalizer = Tokenizers::Normalizers::BertNormalizer.new(
      clean_text: false,
      handle_chinese_chars: false,
      strip_accents: false,
      lowercase: false
    )

    assert_equal false, normalizer.clean_text
    normalizer.clean_text = true
    assert_equal true, normalizer.clean_text

    assert_equal false, normalizer.handle_chinese_chars
    normalizer.handle_chinese_chars = true
    assert_equal true, normalizer.handle_chinese_chars

    assert_equal false, normalizer.strip_accents
    normalizer.strip_accents = true
    assert_equal true, normalizer.strip_accents

    assert_equal false, normalizer.lowercase
    normalizer.lowercase = true
    assert_equal true, normalizer.lowercase
  end

  def test_lowercase
    normalizer = Tokenizers::Normalizers::Lowercase.new
    assert_instance_of Tokenizers::Normalizers::Lowercase, normalizer
    assert_kind_of Tokenizers::Normalizers::Lowercase, normalizer
  end

  def test_nfc
    normalizer = Tokenizers::Normalizers::NFC.new
    assert_instance_of Tokenizers::Normalizers::NFC, normalizer
    assert_kind_of Tokenizers::Normalizers::NFC, normalizer
  end

  def test_nfd
    normalizer = Tokenizers::Normalizers::NFD.new
    assert_instance_of Tokenizers::Normalizers::NFD, normalizer
    assert_kind_of Tokenizers::Normalizers::NFD, normalizer
  end

  def test_nfkc
    normalizer = Tokenizers::Normalizers::NFKC.new
    assert_instance_of Tokenizers::Normalizers::NFKC, normalizer
    assert_kind_of Tokenizers::Normalizers::NFKC, normalizer
  end

  def test_nfkd
    normalizer = Tokenizers::Normalizers::NFKD.new
    assert_instance_of Tokenizers::Normalizers::NFKD, normalizer
    assert_kind_of Tokenizers::Normalizers::NFKD, normalizer
  end

  def test_nmt
    normalizer = Tokenizers::Normalizers::Nmt.new
    assert_instance_of Tokenizers::Normalizers::Nmt, normalizer
    assert_kind_of Tokenizers::Normalizers::Nmt, normalizer
  end

  def test_replace
    normalizer = Tokenizers::Normalizers::Replace.new('abc', 'xyz')
    assert_instance_of Tokenizers::Normalizers::Replace, normalizer
    assert_kind_of Tokenizers::Normalizers::Replace, normalizer
  end

  def test_prepend
    normalizer = Tokenizers::Normalizers::Prepend.new
    assert_instance_of Tokenizers::Normalizers::Prepend, normalizer
    assert_kind_of Tokenizers::Normalizers::Prepend, normalizer
    assert_equal '_', normalizer.prepend

    normalizer = Tokenizers::Normalizers::Prepend.new(prepend: '-')
    assert_equal '-', normalizer.prepend
  end

  def test_strip
    normalizer = Tokenizers::Normalizers::Strip.new
    assert_instance_of Tokenizers::Normalizers::Strip, normalizer
    assert_kind_of Tokenizers::Normalizers::Strip, normalizer

    normalizer = Tokenizers::Normalizers::Strip.new(left: false, right: false)

    assert_equal false, normalizer.left
    normalizer.left = true
    assert_equal true, normalizer.left

    assert_equal false, normalizer.right
    normalizer.right = true
    assert_equal true, normalizer.right
  end

  def test_strip_accents
    normalizer = Tokenizers::Normalizers::StripAccents.new
    assert_instance_of Tokenizers::Normalizers::StripAccents, normalizer
    assert_kind_of Tokenizers::Normalizers::StripAccents, normalizer
  end
end
