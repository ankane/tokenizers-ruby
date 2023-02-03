require_relative "test_helper"

class DecoderTest < Minitest::Test
  def test_bpe_decoder
    decoder = Tokenizers::Decoders::BPEDecoder.new
    assert_instance_of Tokenizers::Decoders::BPEDecoder, decoder
    assert_kind_of Tokenizers::Decoders::Decoder, decoder

    decoder = Tokenizers::Decoders::BPEDecoder.new(suffix: "</end>")
    assert_instance_of Tokenizers::Decoders::BPEDecoder, decoder
    assert_kind_of Tokenizers::Decoders::Decoder, decoder
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

    Tokenizers::Decoders::CTC.new(
      pad_token: "<mypad>",
      word_delimiter_token: "+",
      cleanup: false
    )
  end

  def test_metaspace
    decoder = Tokenizers::Decoders::Metaspace.new
    assert_instance_of Tokenizers::Decoders::Metaspace, decoder
    assert_kind_of Tokenizers::Decoders::Decoder, decoder

    Tokenizers::Decoders::Metaspace.new(replacement: "+", add_prefix_space: false)
  end

  def test_word_piece
    decoder = Tokenizers::Decoders::WordPiece.new
    assert_instance_of Tokenizers::Decoders::WordPiece, decoder
    assert_kind_of Tokenizers::Decoders::Decoder, decoder

    Tokenizers::Decoders::WordPiece.new(prefix: "+", cleanup: false)
  end
end
