require_relative "test_helper"

class DecoderTest < Minitest::Test
  def test_bpe_decoder
    decoder = Tokenizers::Decoders::BPEDecoder.new
    assert_instance_of Tokenizers::Decoders::BPEDecoder, decoder
    assert_kind_of Tokenizers::Decoders::Decoder, decoder

    decoder = Tokenizers::Decoders::BPEDecoder.new(suffix: "</end>")

    assert_equal "</end>", decoder.suffix
    decoder.suffix = "</w>"
    assert_equal "</w>", decoder.suffix
  end

  def test_byte_level
    decoder = Tokenizers::Decoders::ByteLevel.new
    assert_instance_of Tokenizers::Decoders::ByteLevel, decoder
    assert_kind_of Tokenizers::Decoders::Decoder, decoder
  end

  def test_ctc
    decoder = Tokenizers::Decoders::CTC.new
    assert_instance_of Tokenizers::Decoders::CTC, decoder
    assert_kind_of Tokenizers::Decoders::Decoder, decoder

    decoder = Tokenizers::Decoders::CTC.new(
      pad_token: "<mypad>",
      word_delimiter_token: "+",
      cleanup: false
    )

    assert_equal "<mypad>", decoder.pad_token
    decoder.pad_token = "<pad>"
    assert_equal "<pad>", decoder.pad_token

    assert_equal "+", decoder.word_delimiter_token
    decoder.word_delimiter_token = "|"
    assert_equal "|", decoder.word_delimiter_token

    assert_equal false, decoder.cleanup
    decoder.cleanup = true
    assert_equal true, decoder.cleanup
  end

  def test_metaspace
    decoder = Tokenizers::Decoders::Metaspace.new
    assert_instance_of Tokenizers::Decoders::Metaspace, decoder
    assert_kind_of Tokenizers::Decoders::Decoder, decoder

    decoder = Tokenizers::Decoders::Metaspace.new(replacement: "+", add_prefix_space: false)

    assert_equal "+", decoder.replacement
    decoder.replacement = "\u2581"
    assert_equal "\u2581", decoder.replacement

    assert_equal false, decoder.add_prefix_space
    decoder.add_prefix_space = true
    assert_equal true, decoder.add_prefix_space
  end

  def test_word_piece
    decoder = Tokenizers::Decoders::WordPiece.new
    assert_instance_of Tokenizers::Decoders::WordPiece, decoder
    assert_kind_of Tokenizers::Decoders::Decoder, decoder

    decoder = Tokenizers::Decoders::WordPiece.new(prefix: "+", cleanup: false)

    assert_equal "+", decoder.prefix
    decoder.prefix = "\u2581"
    assert_equal "\u2581", decoder.prefix

    assert_equal false, decoder.cleanup
    decoder.cleanup = true
    assert_equal true, decoder.cleanup
  end
end
