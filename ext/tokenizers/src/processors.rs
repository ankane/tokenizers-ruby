use std::sync::Arc;
use std::sync::RwLock;

use magnus::{
    data_type_builder, function, value::Lazy, Class, DataType, DataTypeFunctions, Module, Object,
    RArray, RClass, RModule, Ruby, TryConvert, TypedData, Value,
};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tk::processors::bert::BertProcessing;
use tk::processors::byte_level::ByteLevel;
use tk::processors::roberta::RobertaProcessing;
use tk::processors::template::{SpecialToken, Template};
use tk::processors::PostProcessorWrapper;
use tk::{Encoding, PostProcessor};

use super::{RbResult, PROCESSORS};

#[derive(DataTypeFunctions, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RbPostProcessor {
    pub processor: RbPostProcessorTypeWrapper,
}

impl RbPostProcessor {
    pub fn new(processor: RbPostProcessorTypeWrapper) -> Self {
        RbPostProcessor { processor }
    }
}

impl<I> From<I> for RbPostProcessor
where
    I: Into<PostProcessorWrapper>,
{
    fn from(processor: I) -> Self {
        RbPostProcessor {
            processor: processor.into().into(),
        }
    }
}

impl PostProcessor for RbPostProcessor {
    fn added_tokens(&self, is_pair: bool) -> usize {
        self.processor.added_tokens(is_pair)
    }

    fn process_encodings(
        &self,
        encodings: Vec<Encoding>,
        add_special_tokens: bool,
    ) -> tk::Result<Vec<Encoding>> {
        self.processor
            .process_encodings(encodings, add_special_tokens)
    }
}

#[derive(Clone)]
pub(crate) enum RbPostProcessorTypeWrapper {
    Sequence(Vec<Arc<RwLock<PostProcessorWrapper>>>),
    Single(Arc<RwLock<PostProcessorWrapper>>),
}

impl PostProcessor for RbPostProcessorTypeWrapper {
    fn added_tokens(&self, is_pair: bool) -> usize {
        match self {
            RbPostProcessorTypeWrapper::Single(inner) => inner
                .read()
                .expect("RwLock synchronisation primitive is poisoned, cannot get subtype of RbPostProcessor")
                .added_tokens(is_pair),
            RbPostProcessorTypeWrapper::Sequence(inner) => inner.iter().map(|p| {
                p.read()
                    .expect("RwLock synchronisation primitive is poisoned, cannot get subtype of RbPostProcessor")
                    .added_tokens(is_pair)
            }).sum::<usize>(),
        }
    }

    fn process_encodings(
        &self,
        mut encodings: Vec<Encoding>,
        add_special_tokens: bool,
    ) -> tk::Result<Vec<Encoding>> {
        match self {
            RbPostProcessorTypeWrapper::Single(inner) => inner
                .read()
                .expect("RwLock synchronisation primitive is poisoned, cannot get subtype of RbPreTokenizer")
                .process_encodings(encodings, add_special_tokens),
            RbPostProcessorTypeWrapper::Sequence(inner) => {
                for processor in inner.iter() {
                    encodings = processor
                        .read()
                        .expect("RwLock synchronisation primitive is poisoned, cannot get subtype of RbPreTokenizer")
                        .process_encodings(encodings, add_special_tokens)?;
                }
                Ok(encodings)
            },
        }
    }
}

impl<'de> Deserialize<'de> for RbPostProcessorTypeWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wrapper = PostProcessorWrapper::deserialize(deserializer)?;
        Ok(wrapper.into())
    }
}

impl Serialize for RbPostProcessorTypeWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            RbPostProcessorTypeWrapper::Sequence(seq) => {
                let mut ser = serializer.serialize_struct("Sequence", 2)?;
                ser.serialize_field("type", "Sequence")?;
                ser.serialize_field("processors", seq)?;
                ser.end()
            }
            RbPostProcessorTypeWrapper::Single(inner) => inner.serialize(serializer),
        }
    }
}

impl<I> From<I> for RbPostProcessorTypeWrapper
where
    I: Into<PostProcessorWrapper>,
{
    fn from(processor: I) -> Self {
        let processor = processor.into();
        match processor {
            PostProcessorWrapper::Sequence(seq) => RbPostProcessorTypeWrapper::Sequence(
                seq.into_iter().map(|p| Arc::new(RwLock::new(p))).collect(),
            ),
            _ => RbPostProcessorTypeWrapper::Single(Arc::new(RwLock::new(processor.clone()))),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RbSpecialToken(SpecialToken);

impl From<RbSpecialToken> for SpecialToken {
    fn from(v: RbSpecialToken) -> Self {
        v.0
    }
}

impl TryConvert for RbSpecialToken {
    fn try_convert(ob: Value) -> RbResult<Self> {
        if let Ok(v) = <(String, u32)>::try_convert(ob) {
            Ok(Self(v.into()))
        } else if let Ok(v) = <(u32, String)>::try_convert(ob) {
            Ok(Self(v.into()))
        } else {
            todo!()
        }
    }
}

#[derive(Clone, Debug)]
pub struct RbTemplate(Template);

impl From<RbTemplate> for Template {
    fn from(v: RbTemplate) -> Self {
        v.0
    }
}

impl TryConvert for RbTemplate {
    fn try_convert(ob: Value) -> RbResult<Self> {
        if let Ok(s) = String::try_convert(ob) {
            Ok(Self(
                s.try_into().unwrap(), //.map_err(RbError::from)?,
            ))
        } else if let Ok(s) = <Vec<String>>::try_convert(ob) {
            Ok(Self(
                s.try_into().unwrap(), //.map_err(RbError::from)?,
            ))
        } else {
            todo!()
        }
    }
}

pub struct RbBertProcessing {}

impl RbBertProcessing {
    pub fn new(sep: (String, u32), cls: (String, u32)) -> RbPostProcessor {
        BertProcessing::new(sep, cls).into()
    }
}

pub struct RbByteLevel {}

impl RbByteLevel {
    pub fn new(trim_offsets: Option<bool>) -> RbPostProcessor {
        let mut byte_level = ByteLevel::default();

        if let Some(to) = trim_offsets {
            byte_level = byte_level.trim_offsets(to);
        }
        byte_level.into()
    }
}

pub struct RbRobertaProcessing {}

impl RbRobertaProcessing {
    fn new(
        sep: (String, u32),
        cls: (String, u32),
        trim_offsets: bool,
        add_prefix_space: bool,
    ) -> RbPostProcessor {
        let proc = RobertaProcessing::new(sep, cls)
            .trim_offsets(trim_offsets)
            .add_prefix_space(add_prefix_space);
        proc.into()
    }
}

pub struct RbTemplateProcessing {}

impl RbTemplateProcessing {
    pub fn new(
        single: Option<RbTemplate>,
        pair: Option<RbTemplate>,
        special_tokens: Option<Vec<(String, u32)>>,
    ) -> RbResult<RbPostProcessor> {
        let mut builder = tk::processors::template::TemplateProcessing::builder();

        if let Some(seq) = single {
            builder.single(seq.into());
        }
        if let Some(seq) = pair {
            builder.pair(seq.into());
        }
        if let Some(sp) = special_tokens {
            builder.special_tokens(sp);
        }
        let processor = builder.build().unwrap(); //.map_err(RbError::from)?;

        Ok(processor.into())
    }
}

pub struct RbSequence {}

impl RbSequence {
    fn new(processors_rb: RArray) -> RbResult<RbPostProcessor> {
        let mut processors = Vec::with_capacity(processors_rb.len());
        for n in processors_rb {
            let processor = <&RbPostProcessor>::try_convert(n)?;
            match &processor.processor {
                RbPostProcessorTypeWrapper::Sequence(inner) => {
                    processors.extend(inner.iter().cloned())
                }
                RbPostProcessorTypeWrapper::Single(inner) => processors.push(inner.clone()),
            }
        }
        Ok(RbPostProcessor::new(RbPostProcessorTypeWrapper::Sequence(
            processors,
        )))
    }
}

unsafe impl TypedData for RbPostProcessor {
    fn class(ruby: &Ruby) -> RClass {
        static CLASS: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby
                .get_inner(&PROCESSORS)
                .const_get("PostProcessor")
                .unwrap();
            class.undef_default_alloc_func();
            class
        });
        ruby.get_inner(&CLASS)
    }

    fn data_type() -> &'static DataType {
        static DATA_TYPE: DataType =
            data_type_builder!(RbPostProcessor, "Tokenizers::Processors::PostProcessor").build();
        &DATA_TYPE
    }

    fn class_for(ruby: &Ruby, value: &Self) -> RClass {
        static BERT_PROCESSING: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby
                .get_inner(&PROCESSORS)
                .const_get("BertProcessing")
                .unwrap();
            class.undef_default_alloc_func();
            class
        });
        static BYTE_LEVEL: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby.get_inner(&PROCESSORS).const_get("ByteLevel").unwrap();
            class.undef_default_alloc_func();
            class
        });
        static ROBERTA_PROCESSING: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby
                .get_inner(&PROCESSORS)
                .const_get("RobertaProcessing")
                .unwrap();
            class.undef_default_alloc_func();
            class
        });
        static TEMPLATE_PROCESSING: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby
                .get_inner(&PROCESSORS)
                .const_get("TemplateProcessing")
                .unwrap();
            class.undef_default_alloc_func();
            class
        });
        static SEQUENCE: Lazy<RClass> = Lazy::new(|ruby| {
            let class: RClass = ruby.get_inner(&PROCESSORS).const_get("Sequence").unwrap();
            class.undef_default_alloc_func();
            class
        });
        match &value.processor {
            RbPostProcessorTypeWrapper::Single(inner) => match &*inner.read().unwrap() {
                PostProcessorWrapper::Bert(_) => ruby.get_inner(&BERT_PROCESSING),
                PostProcessorWrapper::ByteLevel(_) => ruby.get_inner(&BYTE_LEVEL),
                PostProcessorWrapper::Roberta(_) => ruby.get_inner(&ROBERTA_PROCESSING),
                PostProcessorWrapper::Template(_) => ruby.get_inner(&TEMPLATE_PROCESSING),
                _ => todo!(),
            },
            RbPostProcessorTypeWrapper::Sequence(_) => ruby.get_inner(&SEQUENCE),
        }
    }
}

pub fn init_processors(ruby: &Ruby, module: &RModule) -> RbResult<()> {
    let post_processor = module.define_class("PostProcessor", ruby.class_object())?;

    let class = module.define_class("BertProcessing", post_processor)?;
    class.define_singleton_method("new", function!(RbBertProcessing::new, 2))?;

    let class = module.define_class("ByteLevel", post_processor)?;
    class.define_singleton_method("_new", function!(RbByteLevel::new, 1))?;

    let class = module.define_class("RobertaProcessing", post_processor)?;
    class.define_singleton_method("_new", function!(RbRobertaProcessing::new, 4))?;

    let class = module.define_class("TemplateProcessing", post_processor)?;
    class.define_singleton_method("_new", function!(RbTemplateProcessing::new, 3))?;

    let class = module.define_class("Sequence", post_processor)?;
    class.define_singleton_method("_new", function!(RbSequence::new, 1))?;

    Ok(())
}
