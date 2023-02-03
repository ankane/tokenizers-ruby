require_relative "test_helper"

class DecoderTest < Minitest::Test
  def test_bpe_decoder
    decoder = Tokenizers::BPEDecoder.new
    assert_instance_of Tokenizers::BPEDecoder, decoder
    assert_kind_of Tokenizers::Decoder, decoder

    decoder = Tokenizers::BPEDecoder.new(suffix: "</end>")
    assert_instance_of Tokenizers::BPEDecoder, decoder
    assert_kind_of Tokenizers::Decoder, decoder
  end

  def test_byte_level
    decoder = Tokenizers::ByteLevelDecoder.new
    assert_instance_of Tokenizers::ByteLevelDecoder, decoder
    assert_kind_of Tokenizers::Decoder, decoder
  end

  def test_ctc
    decoder = Tokenizers::CTC.new
    assert_instance_of Tokenizers::CTC, decoder
    assert_kind_of Tokenizers::Decoder, decoder

    decoder = Tokenizers::CTC.new(pad_token: "<mypad>")
    assert_instance_of Tokenizers::CTC, decoder
    assert_kind_of Tokenizers::Decoder, decoder

    decoder = Tokenizers::CTC.new(word_delimiter_token: "+")
    assert_instance_of Tokenizers::CTC, decoder
    assert_kind_of Tokenizers::Decoder, decoder

    decoder = Tokenizers::CTC.new(cleanup: false)
    assert_instance_of Tokenizers::CTC, decoder
    assert_kind_of Tokenizers::Decoder, decoder
  end

  def test_metaspace
    decoder = Tokenizers::MetaspaceDecoder.new
    assert_instance_of Tokenizers::MetaspaceDecoder, decoder
    assert_kind_of Tokenizers::Decoder, decoder

    decoder = Tokenizers::MetaspaceDecoder.new(replacement: "+")
    assert_instance_of Tokenizers::MetaspaceDecoder, decoder
    assert_kind_of Tokenizers::Decoder, decoder

    decoder = Tokenizers::MetaspaceDecoder.new(add_prefix_space: false)
    assert_instance_of Tokenizers::MetaspaceDecoder, decoder
    assert_kind_of Tokenizers::Decoder, decoder
  end

  def test_word_piece
    decoder = Tokenizers::WordPieceDecoder.new
    assert_instance_of Tokenizers::WordPieceDecoder, decoder
    assert_kind_of Tokenizers::Decoder, decoder

    decoder = Tokenizers::WordPieceDecoder.new(prefix: "+")
    assert_instance_of Tokenizers::WordPieceDecoder, decoder
    assert_kind_of Tokenizers::Decoder, decoder

    decoder = Tokenizers::WordPieceDecoder.new(cleanup: false)
    assert_instance_of Tokenizers::WordPieceDecoder, decoder
    assert_kind_of Tokenizers::Decoder, decoder
  end
end
