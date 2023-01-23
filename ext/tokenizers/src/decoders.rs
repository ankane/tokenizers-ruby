use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use tk::decoders::bpe::BPEDecoder;
use tk::decoders::DecoderWrapper;
use tk::Decoder;

#[magnus::wrap(class = "Tokenizers::Decoder")]
#[derive(Clone, Deserialize, Serialize)]
pub struct RbDecoder {
    #[serde(flatten)]
    pub(crate) decoder: RbDecoderWrapper,
}

impl Decoder for RbDecoder {
    fn decode_chain(&self, tokens: Vec<String>) -> tk::Result<Vec<String>> {
        self.decoder.decode_chain(tokens)
    }
}

#[magnus::wrap(class = "Tokenizers::BPEDecoder")]
pub struct RbBPEDecoder {
    pub decoder: BPEDecoder,
}

impl RbBPEDecoder {
    pub fn new() -> RbDecoder {
        BPEDecoder::default().into()
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub(crate) enum RbDecoderWrapper {
    // Custom(Arc<RwLock<CustomDecoder>>),
    Wrapped(Arc<RwLock<DecoderWrapper>>),
}

impl<I> From<I> for RbDecoderWrapper
where
    I: Into<DecoderWrapper>,
{
    fn from(norm: I) -> Self {
        RbDecoderWrapper::Wrapped(Arc::new(RwLock::new(norm.into())))
    }
}

impl<I> From<I> for RbDecoder
where
    I: Into<DecoderWrapper>,
{
    fn from(dec: I) -> Self {
        RbDecoder {
            decoder: dec.into().into(),
        }
    }
}

impl Decoder for RbDecoderWrapper {
    fn decode_chain(&self, tokens: Vec<String>) -> tk::Result<Vec<String>> {
        match self {
            RbDecoderWrapper::Wrapped(inner) => inner.read().unwrap().decode_chain(tokens),
            // RbDecoderWrapper::Custom(inner) => inner.read().unwrap().decode_chain(tokens),
        }
    }
}
