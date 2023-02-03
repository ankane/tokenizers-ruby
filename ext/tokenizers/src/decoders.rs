use std::sync::{Arc, RwLock};

use magnus::typed_data::DataTypeBuilder;
use magnus::{
    function, memoize, method, Class, DataType, DataTypeFunctions, Module, Object, RClass, RModule,
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

use super::RbResult;

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

macro_rules! getter {
    ($self: ident, $variant: ident, $($name: tt)+) => {{
        let decoder = &$self.decoder;
        let RbDecoderWrapper::Wrapped(ref wrap) = decoder;
        if let DecoderWrapper::$variant(ref dec) = *wrap.read().unwrap() {
            dec.$($name)+
        } else {
            unreachable!()
        }
    }};
}

macro_rules! setter {
    ($self: ident, $variant: ident, $name: ident, $value: expr) => {{
        let decoder = &$self.decoder;
        let RbDecoderWrapper::Wrapped(ref wrap) = decoder;
        if let DecoderWrapper::$variant(ref mut dec) = *wrap.write().unwrap() {
            dec.$name = $value;
        }
    }};
    ($self: ident, $variant: ident, @$name: ident, $value: expr) => {{
        let decoder = &$self.decoder;
        let RbDecoderWrapper::Wrapped(ref wrap) = decoder;
        if let DecoderWrapper::$variant(ref mut dec) = *wrap.write().unwrap() {
            dec.$name($value);
        }
    }};
}
impl RbDecoder {
    pub fn bpe_suffix(&self) -> String {
        getter!(self, BPE, suffix.clone())
    }

    pub fn bpe_set_suffix(&self, suffix: String) {
        setter!(self, BPE, suffix, suffix);
    }

    pub fn ctc_cleanup(&self) -> bool {
        getter!(self, CTC, cleanup)
    }

    pub fn ctc_set_cleanup(&self, cleanup: bool) {
        setter!(self, CTC, cleanup, cleanup);
    }

    pub fn ctc_pad_token(&self) -> String {
        getter!(self, CTC, pad_token.clone())
    }

    pub fn ctc_set_pad_token(&self, pad_token: String) {
        setter!(self, CTC, pad_token, pad_token);
    }

    pub fn ctc_word_delimiter_token(&self) -> String {
        getter!(self, CTC, word_delimiter_token.clone())
    }

    pub fn ctc_set_word_delimiter_token(&self, word_delimiter_token: String) {
        setter!(self, CTC, word_delimiter_token, word_delimiter_token);
    }

    pub fn metaspace_replacement(&self) -> char {
        getter!(self, Metaspace, get_replacement().clone())
    }

    pub fn metaspace_set_replacement(&self, replacement: char) {
        let decoder = &self.decoder;
        let RbDecoderWrapper::Wrapped(ref wrap) = decoder;
        if let DecoderWrapper::Metaspace(ref mut dec) = *wrap.write().unwrap() {
            dec.set_replacement(replacement)
        }
    }

    pub fn metaspace_add_prefix_space(&self) -> bool {
        getter!(self, Metaspace, add_prefix_space)
    }

    pub fn metaspace_set_add_prefix_space(&self, add_prefix_space: bool) {
        setter!(self, Metaspace, add_prefix_space, add_prefix_space);
    }

    pub fn word_piece_cleanup(&self) -> bool {
        getter!(self, WordPiece, cleanup)
    }

    pub fn word_piece_set_cleanup(&self, cleanup: bool) {
        setter!(self, WordPiece, cleanup, cleanup);
    }

    pub fn word_piece_prefix(&self) -> String {
        getter!(self, WordPiece, prefix.clone())
    }

    pub fn word_piece_set_prefix(&self, prefix: String) {
        setter!(self, WordPiece, prefix, prefix);
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
          let class: RClass = crate::decoders().const_get("Decoder").unwrap();
          class.undef_alloc_func();
          class
        })
    }

    fn data_type() -> &'static DataType {
        memoize!(DataType: DataTypeBuilder::<RbDecoder>::new("Tokenizers::Decoders::Decoder").build())
    }

    fn class_for(value: &Self) -> RClass {
        match &value.decoder {
            RbDecoderWrapper::Wrapped(inner) => match *inner.read().unwrap() {
                DecoderWrapper::BPE(_) => *memoize!(RClass: {
                    let class: RClass = crate::decoders().const_get("BPEDecoder").unwrap();
                    class.undef_alloc_func();
                    class
                }),
                DecoderWrapper::ByteLevel(_) => *memoize!(RClass: {
                    let class: RClass = crate::decoders().const_get("ByteLevel").unwrap();
                    class.undef_alloc_func();
                    class
                }),
                DecoderWrapper::CTC(_) => *memoize!(RClass: {
                    let class: RClass = crate::decoders().const_get("CTC").unwrap();
                    class.undef_alloc_func();
                    class
                }),
                DecoderWrapper::Metaspace(_) => *memoize!(RClass: {
                    let class: RClass = crate::decoders().const_get("Metaspace").unwrap();
                    class.undef_alloc_func();
                    class
                }),
                DecoderWrapper::WordPiece(_) => *memoize!(RClass: {
                    let class: RClass = crate::decoders().const_get("WordPiece").unwrap();
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
    class.define_method("suffix", method!(RbDecoder::bpe_suffix, 0))?;
    class.define_method("suffix=", method!(RbDecoder::bpe_set_suffix, 1))?;

    let class = module.define_class("ByteLevel", decoder)?;
    class.define_singleton_method("new", function!(RbByteLevelDecoder::new, 0))?;

    let class = module.define_class("CTC", decoder)?;
    class.define_singleton_method("_new", function!(RbCTC::new, 3))?;
    class.define_method("cleanup", method!(RbDecoder::ctc_cleanup, 0))?;
    class.define_method("cleanup=", method!(RbDecoder::ctc_set_cleanup, 1))?;
    class.define_method("pad_token", method!(RbDecoder::ctc_pad_token, 0))?;
    class.define_method("pad_token=", method!(RbDecoder::ctc_set_pad_token, 1))?;
    class.define_method("word_delimiter_token", method!(RbDecoder::ctc_word_delimiter_token, 0))?;
    class.define_method("word_delimiter_token=", method!(RbDecoder::ctc_set_word_delimiter_token, 1))?;

    let class = module.define_class("Metaspace", decoder)?;
    class.define_singleton_method("_new", function!(RbMetaspaceDecoder::new, 2))?;
    class.define_method("add_prefix_space", method!(RbDecoder::metaspace_add_prefix_space, 0))?;
    class.define_method("add_prefix_space=", method!(RbDecoder::metaspace_set_add_prefix_space, 1))?;
    class.define_method("replacement", method!(RbDecoder::metaspace_replacement, 0))?;
    class.define_method("replacement=", method!(RbDecoder::metaspace_set_replacement, 1))?;

    let class = module.define_class("WordPiece", decoder)?;
    class.define_singleton_method("_new", function!(RbWordPieceDecoder::new, 2))?;
    class.define_method("cleanup", method!(RbDecoder::word_piece_cleanup, 0))?;
    class.define_method("cleanup=", method!(RbDecoder::word_piece_set_cleanup, 1))?;
    class.define_method("prefix", method!(RbDecoder::word_piece_prefix, 0))?;
    class.define_method("prefix=", method!(RbDecoder::word_piece_set_prefix, 1))?;

    Ok(())
}
