require_relative "test_helper"

class DecoderTest < Minitest::Test
  def test_bpe_decoder
    decoder = Tokenizers::BPEDecoder.new
    assert_instance_of Tokenizers::BPEDecoder, decoder
    assert_kind_of Tokenizers::Decoder, decoder
  end
end
