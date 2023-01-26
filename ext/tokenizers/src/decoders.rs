use std::sync::{Arc, RwLock};

use magnus::typed_data::DataTypeBuilder;
use magnus::{memoize, Class, DataType, DataTypeFunctions, Module, RClass, TypedData};
use serde::{Deserialize, Serialize};
use tk::decoders::bpe::BPEDecoder;
use tk::decoders::DecoderWrapper;
use tk::Decoder;

use super::module;

#[derive(DataTypeFunctions, Clone, Deserialize, Serialize)]
pub struct RbDecoder {
    #[serde(flatten)]
    pub(crate) decoder: RbDecoderWrapper,
}

impl Decoder for RbDecoder {
    fn decode_chain(&self, tokens: Vec<String>) -> tk::Result<Vec<String>> {
        self.decoder.decode_chain(tokens)
    }
}

pub struct RbBPEDecoder {}

impl RbBPEDecoder {
    // TODO return BPEDecoder class
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

unsafe impl TypedData for RbDecoder {
    fn class() -> RClass {
        *memoize!(RClass: {
          let class: RClass = module().const_get("Decoder").unwrap();
          class.undef_alloc_func();
          class
        })
    }

    fn data_type() -> &'static DataType {
        memoize!(DataType: DataTypeBuilder::<RbDecoder>::new("Tokenizers::Decoder").build())
    }

    fn class_for(value: &Self) -> RClass {
        match &value.decoder {
            RbDecoderWrapper::Wrapped(inner) => match *inner.read().unwrap() {
                DecoderWrapper::BPE(_) => *memoize!(RClass: {
                    let class: RClass = module().const_get("BPEDecoder").unwrap();
                    class.undef_alloc_func();
                    class
                }),
                _ => Self::class(),
            },
        }
    }
}
