use std::sync::{Arc, RwLock};

use magnus::typed_data::DataTypeBuilder;
use magnus::{
    function, memoize, Class, DataType, DataTypeFunctions, Module, Object, RClass, RModule,
    TypedData,
};
use serde::{Deserialize, Serialize};
use tk::decoders::bpe::BPEDecoder;
use tk::decoders::byte_level::ByteLevel;
use tk::decoders::ctc::CTC;
use tk::decoders::metaspace::Metaspace;
use tk::decoders::wordpiece::WordPiece;
use tk::decoders::DecoderWrapper;
use tk::Decoder;

use super::{module, RbResult};

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
    pub fn new(suffix: String) -> RbDecoder {
        BPEDecoder::new(suffix).into()
    }
}

pub struct RbByteLevelDecoder {}

impl RbByteLevelDecoder {
    pub fn new() -> RbDecoder {
        ByteLevel::default().into()
    }
}

pub struct RbCTC {}

impl RbCTC {
    pub fn new(pad_token: String, word_delimiter_token: String, cleanup: bool) -> RbDecoder {
        CTC::new(pad_token, word_delimiter_token, cleanup).into()
    }
}

pub struct RbMetaspaceDecoder {}

impl RbMetaspaceDecoder {
    pub fn new(replacement: char, add_prefix_space: bool) -> RbDecoder {
        Metaspace::new(replacement, add_prefix_space).into()
    }
}

pub struct RbWordPieceDecoder {}

impl RbWordPieceDecoder {
    pub fn new(prefix: String, cleanup: bool) -> RbDecoder {
        WordPiece::new(prefix, cleanup).into()
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
                DecoderWrapper::ByteLevel(_) => *memoize!(RClass: {
                    let class: RClass = module().const_get("ByteLevelDecoder").unwrap();
                    class.undef_alloc_func();
                    class
                }),
                DecoderWrapper::CTC(_) => *memoize!(RClass: {
                    let class: RClass = module().const_get("CTC").unwrap();
                    class.undef_alloc_func();
                    class
                }),
                DecoderWrapper::Metaspace(_) => *memoize!(RClass: {
                    let class: RClass = module().const_get("MetaspaceDecoder").unwrap();
                    class.undef_alloc_func();
                    class
                }),
                DecoderWrapper::WordPiece(_) => *memoize!(RClass: {
                    let class: RClass = module().const_get("WordPieceDecoder").unwrap();
                    class.undef_alloc_func();
                    class
                }),
                _ => todo!(),
            },
        }
    }
}

pub fn decoders(module: &RModule) -> RbResult<()> {
    let decoder = module.define_class("Decoder", Default::default())?;

    let class = module.define_class("BPEDecoder", decoder)?;
    class.define_singleton_method("_new", function!(RbBPEDecoder::new, 1))?;

    let class = module.define_class("ByteLevelDecoder", decoder)?;
    class.define_singleton_method("new", function!(RbByteLevelDecoder::new, 0))?;

    let class = module.define_class("CTC", decoder)?;
    class.define_singleton_method("_new", function!(RbCTC::new, 3))?;

    let class = module.define_class("MetaspaceDecoder", decoder)?;
    class.define_singleton_method("_new", function!(RbMetaspaceDecoder::new, 2))?;

    let class = module.define_class("WordPieceDecoder", decoder)?;
    class.define_singleton_method("_new", function!(RbWordPieceDecoder::new, 2))?;

    Ok(())
}
