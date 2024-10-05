use std::sync::{Arc, RwLock};

use crate::pre_tokenizers::from_string;
use magnus::value::Lazy;
use magnus::{
    data_type_builder, function, method, Class, DataType, DataTypeFunctions, Module, Object,
    RClass, RModule, Ruby, TypedData,
};
use serde::{Deserialize, Serialize};
use tk::decoders::bpe::BPEDecoder;
use tk::decoders::byte_fallback::ByteFallback;
use tk::decoders::byte_level::ByteLevel;
use tk::decoders::ctc::CTC;
use tk::decoders::fuse::Fuse;
use tk::decoders::metaspace::{Metaspace, PrependScheme};
use tk::decoders::strip::Strip;
use tk::decoders::wordpiece::WordPiece;
use tk::decoders::DecoderWrapper;
use tk::normalizers::replace::Replace;
use tk::Decoder;

use super::utils::*;
use super::{RbError, RbResult, DECODERS};

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

impl RbDecoder {
    pub fn decode(&self, tokens: Vec<String>) -> RbResult<String> {
        self.decoder.decode(tokens).map_err(RbError::from)
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

    fn strip_content(&self) -> char {
        getter!(self, Strip, content)
    }

    fn strip_set_content(&self, content: char) {
        setter!(self, Strip, content, content)
    }

    fn strip_start(&self) -> usize {
        getter!(self, Strip, start)
    }

    fn strip_set_start(&self, start: usize) {
        setter!(self, Strip, start, start)
    }

    fn strip_stop(&self) -> usize {
        getter!(self, Strip, stop)
    }

    fn strip_set_stop(&self, stop: usize) {
        setter!(self, Strip, stop, stop)
    }

    pub fn metaspace_replacement(&self) -> char {
        getter!(self, Metaspace, get_replacement().clone())
    }

    pub fn metaspace_set_replacement(&self, replacement: char) {
        setter!(self, Metaspace, @set_replacement, replacement);
    }

    pub fn metaspace_split(&self) -> bool {
        getter!(self, Metaspace, get_split())
    }

    pub fn metaspace_set_split(&self, split: bool) {
        setter!(self, Metaspace, @set_split, split);
    }

    pub fn metaspace_prepend_scheme(&self) -> String {
        // Assuming Metaspace has a method to get the prepend_scheme as a string
        let scheme: PrependScheme = getter!(self, Metaspace, get_prepend_scheme());
        match scheme {
            PrependScheme::First => "first",
            PrependScheme::Never => "never",
            PrependScheme::Always => "always",
        }
        .to_string()
    }

    pub fn metaspace_set_prepend_scheme(&self, prepend_scheme: String) -> RbResult<()> {
        let scheme = from_string(prepend_scheme)?;
        setter!(self, Metaspace, @set_prepend_scheme, scheme);
        Ok(())
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

pub struct RbByteFallbackDecoder {}

impl RbByteFallbackDecoder {
    pub fn new() -> RbDecoder {
        ByteFallback::default().into()
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

pub struct RbFuse {}

impl RbFuse {
    pub fn new() -> RbDecoder {
        Fuse::default().into()
    }
}

pub struct RbMetaspaceDecoder {}

impl RbMetaspaceDecoder {
    pub fn new(replacement: char, prepend_scheme: String, split: bool) -> RbResult<RbDecoder> {
        let prepend_scheme = from_string(prepend_scheme)?;
        Ok(Metaspace::new(replacement, prepend_scheme, split).into())
    }
}

pub struct RbReplaceDecoder {}

impl RbReplaceDecoder {
    pub fn new(pattern: RbPattern, content: String) -> RbResult<RbDecoder> {
        Replace::new(pattern, content)
            .map(|v| v.into())
            .map_err(RbError::from)
    }
}

pub struct RbStripDecoder {}

impl RbStripDecoder {
    pub fn new(content: char, start: usize, stop: usize) -> RbDecoder {
        Strip::new(content, start, stop).into()
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
    fn class(ruby: &Ruby) -> RClass {
        static CLASS: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby.get_inner(&DECODERS).const_get("Decoder").unwrap();
            class.undef_default_alloc_func();
            class
        });
        ruby.get_inner(&CLASS)
    }

    fn data_type() -> &'static DataType {
        static DATA_TYPE: DataType =
            data_type_builder!(RbDecoder, "Tokenizers::Decoders::Decoder").build();
        &DATA_TYPE
    }

    fn class_for(ruby: &Ruby, value: &Self) -> RClass {
        static BPE_DECODER: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby.get_inner(&DECODERS).const_get("BPEDecoder").unwrap();
            class.undef_default_alloc_func();
            class
        });
        static BYTE_FALLBACK: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby.get_inner(&DECODERS).const_get("ByteFallback").unwrap();
            class.undef_default_alloc_func();
            class
        });
        static BYTE_LEVEL: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby.get_inner(&DECODERS).const_get("ByteLevel").unwrap();
            class.undef_default_alloc_func();
            class
        });
        static CTC: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby.get_inner(&DECODERS).const_get("CTC").unwrap();
            class.undef_default_alloc_func();
            class
        });
        static FUSE: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby.get_inner(&DECODERS).const_get("Fuse").unwrap();
            class.undef_default_alloc_func();
            class
        });
        static METASPACE: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby.get_inner(&DECODERS).const_get("Metaspace").unwrap();
            class.undef_default_alloc_func();
            class
        });
        static REPLACE: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby.get_inner(&DECODERS).const_get("Replace").unwrap();
            class.undef_default_alloc_func();
            class
        });
        static STRIP: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby.get_inner(&DECODERS).const_get("Strip").unwrap();
            class.undef_default_alloc_func();
            class
        });
        static WORD_PIECE: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby.get_inner(&DECODERS).const_get("WordPiece").unwrap();
            class.undef_default_alloc_func();
            class
        });
        match &value.decoder {
            RbDecoderWrapper::Wrapped(inner) => match *inner.read().unwrap() {
                DecoderWrapper::BPE(_) => ruby.get_inner(&BPE_DECODER),
                DecoderWrapper::ByteFallback(_) => ruby.get_inner(&BYTE_FALLBACK),
                DecoderWrapper::ByteLevel(_) => ruby.get_inner(&BYTE_LEVEL),
                DecoderWrapper::CTC(_) => ruby.get_inner(&CTC),
                DecoderWrapper::Fuse(_) => ruby.get_inner(&FUSE),
                DecoderWrapper::Metaspace(_) => ruby.get_inner(&METASPACE),
                DecoderWrapper::Replace(_) => ruby.get_inner(&REPLACE),
                DecoderWrapper::Strip(_) => ruby.get_inner(&STRIP),
                DecoderWrapper::WordPiece(_) => ruby.get_inner(&WORD_PIECE),
                _ => todo!(),
            },
        }
    }
}

pub fn init_decoders(ruby: &Ruby, module: &RModule) -> RbResult<()> {
    let decoder = module.define_class("Decoder", ruby.class_object())?;
    decoder.define_method("decode", method!(RbDecoder::decode, 1))?;

    let class = module.define_class("BPEDecoder", decoder)?;
    class.define_singleton_method("_new", function!(RbBPEDecoder::new, 1))?;
    class.define_method("suffix", method!(RbDecoder::bpe_suffix, 0))?;
    class.define_method("suffix=", method!(RbDecoder::bpe_set_suffix, 1))?;

    let class = module.define_class("ByteFallback", decoder)?;
    class.define_singleton_method("new", function!(RbByteFallbackDecoder::new, 0))?;

    let class = module.define_class("ByteLevel", decoder)?;
    class.define_singleton_method("new", function!(RbByteLevelDecoder::new, 0))?;

    let class = module.define_class("CTC", decoder)?;
    class.define_singleton_method("_new", function!(RbCTC::new, 3))?;
    class.define_method("cleanup", method!(RbDecoder::ctc_cleanup, 0))?;
    class.define_method("cleanup=", method!(RbDecoder::ctc_set_cleanup, 1))?;
    class.define_method("pad_token", method!(RbDecoder::ctc_pad_token, 0))?;
    class.define_method("pad_token=", method!(RbDecoder::ctc_set_pad_token, 1))?;
    class.define_method(
        "word_delimiter_token",
        method!(RbDecoder::ctc_word_delimiter_token, 0),
    )?;
    class.define_method(
        "word_delimiter_token=",
        method!(RbDecoder::ctc_set_word_delimiter_token, 1),
    )?;

    let class = module.define_class("Fuse", decoder)?;
    class.define_singleton_method("new", function!(RbFuse::new, 0))?;

    let class = module.define_class("Metaspace", decoder)?;
    class.define_singleton_method("_new", function!(RbMetaspaceDecoder::new, 3))?;
    class.define_method(
        "prepend_scheme",
        method!(RbDecoder::metaspace_prepend_scheme, 0),
    )?;
    class.define_method(
        "prepend_scheme=",
        method!(RbDecoder::metaspace_set_prepend_scheme, 1),
    )?;
    class.define_method("replacement", method!(RbDecoder::metaspace_replacement, 0))?;
    class.define_method(
        "replacement=",
        method!(RbDecoder::metaspace_set_replacement, 1),
    )?;
    class.define_method("split", method!(RbDecoder::metaspace_split, 0))?;
    class.define_method("split=", method!(RbDecoder::metaspace_set_split, 1))?;

    let class = module.define_class("Replace", decoder)?;
    class.define_singleton_method("new", function!(RbReplaceDecoder::new, 2))?;

    let class = module.define_class("Strip", decoder)?;
    class.define_singleton_method("_new", function!(RbStripDecoder::new, 3))?;
    class.define_method("content", method!(RbDecoder::strip_content, 0))?;
    class.define_method("content=", method!(RbDecoder::strip_set_content, 1))?;
    class.define_method("start", method!(RbDecoder::strip_start, 0))?;
    class.define_method("start=", method!(RbDecoder::strip_set_start, 1))?;
    class.define_method("stop", method!(RbDecoder::strip_stop, 0))?;
    class.define_method("stop=", method!(RbDecoder::strip_set_stop, 1))?;

    let class = module.define_class("WordPiece", decoder)?;
    class.define_singleton_method("_new", function!(RbWordPieceDecoder::new, 2))?;
    class.define_method("cleanup", method!(RbDecoder::word_piece_cleanup, 0))?;
    class.define_method("cleanup=", method!(RbDecoder::word_piece_set_cleanup, 1))?;
    class.define_method("prefix", method!(RbDecoder::word_piece_prefix, 0))?;
    class.define_method("prefix=", method!(RbDecoder::word_piece_set_prefix, 1))?;

    Ok(())
}
