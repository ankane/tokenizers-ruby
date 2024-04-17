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

  def test_byte_fallback
    decoder = Tokenizers::Decoders::ByteFallback.new
    assert_instance_of Tokenizers::Decoders::ByteFallback, decoder
    assert_kind_of Tokenizers::Decoders::ByteFallback, decoder
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

  def test_fuse
    decoder = Tokenizers::Decoders::Fuse.new
    assert_instance_of Tokenizers::Decoders::Fuse, decoder
    assert_kind_of Tokenizers::Decoders::Fuse, decoder
  end

  def test_metaspace
    decoder = Tokenizers::Decoders::Metaspace.new
    assert_instance_of Tokenizers::Decoders::Metaspace, decoder
    assert_kind_of Tokenizers::Decoders::Decoder, decoder

    decoder = Tokenizers::Decoders::Metaspace.new(replacement: "+", prepend_scheme: "never", split: false)

    assert_equal "+", decoder.replacement
    decoder.replacement = "\u2581"
    assert_equal "\u2581", decoder.replacement

    assert_equal "never", decoder.prepend_scheme
    decoder.prepend_scheme = "always"
    assert_equal "always", decoder.prepend_scheme

    assert_equal false, decoder.split
    decoder.split = true
    assert_equal true, decoder.split
  end

  def test_replace
    decoder = Tokenizers::Decoders::Replace.new('abc', 'xyz')
    assert_instance_of Tokenizers::Decoders::Replace, decoder
    assert_kind_of Tokenizers::Decoders::Replace, decoder
  end

  def test_strip
    decoder = Tokenizers::Decoders::Strip.new
    assert_instance_of Tokenizers::Decoders::Strip, decoder
    assert_kind_of Tokenizers::Decoders::Strip, decoder

    assert_equal " ", decoder.content
    assert_equal 0, decoder.start
    assert_equal 0, decoder.stop

    decoder = Tokenizers::Decoders::Strip.new(
      content: "-",
      start: 4,
      stop: 12
    )

    assert_equal "-", decoder.content
    decoder.content = "_"
    assert_equal "_", decoder.content

    assert_equal 4, decoder.start
    decoder.start = 8
    assert_equal 8, decoder.start

    assert_equal 12, decoder.stop
    decoder.stop = 16
    assert_equal 16, decoder.stop
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
