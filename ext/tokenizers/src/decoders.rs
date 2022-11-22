use tk::decoders::bpe::BPEDecoder;

#[magnus::wrap(class = "Tokenizers::BPEDecoder")]
pub struct RbBPEDecoder {
    pub decoder: BPEDecoder,
}

impl RbBPEDecoder {
    pub fn new() -> Self {
        RbBPEDecoder {
            decoder: BPEDecoder::default(),
        }
    }
}
