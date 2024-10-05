use std::sync::Arc;

use magnus::{
    data_type_builder, function, value::Lazy, Class, DataType, DataTypeFunctions, Module, Object,
    RClass, RModule, Ruby, TryConvert, TypedData, Value,
};
use serde::{Deserialize, Serialize};
use tk::processors::bert::BertProcessing;
use tk::processors::byte_level::ByteLevel;
use tk::processors::roberta::RobertaProcessing;
use tk::processors::template::{SpecialToken, Template};
use tk::processors::PostProcessorWrapper;
use tk::{Encoding, PostProcessor};

use super::{RbResult, PROCESSORS};

#[derive(DataTypeFunctions, Clone, Deserialize, Serialize)]
pub struct RbPostProcessor {
    #[serde(flatten)]
    pub processor: Arc<PostProcessorWrapper>,
}

impl RbPostProcessor {
    pub fn new(processor: Arc<PostProcessorWrapper>) -> Self {
        RbPostProcessor { processor }
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
        RbPostProcessor::new(Arc::new(BertProcessing::new(sep, cls).into()))
    }
}

pub struct RbByteLevel {}

impl RbByteLevel {
    pub fn new(trim_offsets: Option<bool>) -> RbPostProcessor {
        let mut byte_level = ByteLevel::default();

        if let Some(to) = trim_offsets {
            byte_level = byte_level.trim_offsets(to);
        }
        RbPostProcessor::new(Arc::new(byte_level.into()))
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
        RbPostProcessor::new(Arc::new(proc.into()))
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

        Ok(RbPostProcessor::new(Arc::new(processor.into())))
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
        match *value.processor {
            PostProcessorWrapper::Bert(_) => ruby.get_inner(&BERT_PROCESSING),
            PostProcessorWrapper::ByteLevel(_) => ruby.get_inner(&BYTE_LEVEL),
            PostProcessorWrapper::Roberta(_) => ruby.get_inner(&ROBERTA_PROCESSING),
            PostProcessorWrapper::Template(_) => ruby.get_inner(&TEMPLATE_PROCESSING),
            _ => todo!(),
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

    Ok(())
}
