use std::sync::{Arc, RwLock};

use magnus::typed_data::DataTypeBuilder;
use magnus::{memoize, Class, DataType, DataTypeFunctions, Module, RClass, TypedData};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use tk::normalizers::{BertNormalizer, NormalizerWrapper};
use tk::{NormalizedString, Normalizer};

use super::module;

#[derive(DataTypeFunctions, Clone, Serialize, Deserialize)]
pub struct RbNormalizer {
    #[serde(flatten)]
    pub(crate) normalizer: RbNormalizerTypeWrapper,
}

impl Normalizer for RbNormalizer {
    fn normalize(&self, normalized: &mut NormalizedString) -> tk::Result<()> {
        self.normalizer.normalize(normalized)
    }
}

pub struct RbBertNormalizer {}

impl RbBertNormalizer {
    pub fn new() -> RbNormalizer {
        BertNormalizer::default().into()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum RbNormalizerWrapper {
    // Custom(CustomNormalizer),
    Wrapped(NormalizerWrapper),
}

impl Serialize for RbNormalizerWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            RbNormalizerWrapper::Wrapped(inner) => inner.serialize(serializer),
            // RbNormalizerWrapper::Custom(inner) => inner.serialize(serializer),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum RbNormalizerTypeWrapper {
    Sequence(Vec<Arc<RwLock<RbNormalizerWrapper>>>),
    Single(Arc<RwLock<RbNormalizerWrapper>>),
}

impl Serialize for RbNormalizerTypeWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            RbNormalizerTypeWrapper::Sequence(seq) => {
                let mut ser = serializer.serialize_struct("Sequence", 2)?;
                ser.serialize_field("type", "Sequence")?;
                ser.serialize_field("normalizers", seq)?;
                ser.end()
            }
            RbNormalizerTypeWrapper::Single(inner) => inner.serialize(serializer),
        }
    }
}

impl<I> From<I> for RbNormalizerWrapper
where
    I: Into<NormalizerWrapper>,
{
    fn from(norm: I) -> Self {
        RbNormalizerWrapper::Wrapped(norm.into())
    }
}

impl<I> From<I> for RbNormalizerTypeWrapper
where
    I: Into<RbNormalizerWrapper>,
{
    fn from(norm: I) -> Self {
        RbNormalizerTypeWrapper::Single(Arc::new(RwLock::new(norm.into())))
    }
}

impl<I> From<I> for RbNormalizer
where
    I: Into<NormalizerWrapper>,
{
    fn from(norm: I) -> Self {
        RbNormalizer {
            normalizer: norm.into().into(),
        }
    }
}

impl Normalizer for RbNormalizerTypeWrapper {
    fn normalize(&self, normalized: &mut NormalizedString) -> tk::Result<()> {
        match self {
            RbNormalizerTypeWrapper::Single(inner) => inner.read().unwrap().normalize(normalized),
            RbNormalizerTypeWrapper::Sequence(inner) => inner
                .iter()
                .try_for_each(|n| n.read().unwrap().normalize(normalized)),
        }
    }
}

impl Normalizer for RbNormalizerWrapper {
    fn normalize(&self, normalized: &mut NormalizedString) -> tk::Result<()> {
        match self {
            RbNormalizerWrapper::Wrapped(inner) => inner.normalize(normalized),
            // RbNormalizerWrapper::Custom(inner) => inner.normalize(normalized),
        }
    }
}

unsafe impl TypedData for RbNormalizer {
    fn class() -> RClass {
        *memoize!(RClass: {
          let class: RClass = module().const_get("Normalizer").unwrap();
          class.undef_alloc_func();
          class
        })
    }

    fn data_type() -> &'static DataType {
        memoize!(DataType: DataTypeBuilder::<RbNormalizer>::new("Tokenizers::Normalizer").build())
    }

    fn class_for(value: &Self) -> RClass {
        match &value.normalizer {
            RbNormalizerTypeWrapper::Sequence(_seq) => todo!(),
            RbNormalizerTypeWrapper::Single(inner) => match &*inner.read().unwrap() {
                RbNormalizerWrapper::Wrapped(wrapped) => match &wrapped {
                    NormalizerWrapper::BertNormalizer(_) => *memoize!(RClass: {
                        let class: RClass = module().const_get("BertNormalizer").unwrap();
                        class.undef_alloc_func();
                        class
                    }),
                    _ => todo!(),
                },
            },
        }
    }
}
