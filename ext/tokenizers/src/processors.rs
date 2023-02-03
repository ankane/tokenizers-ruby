use std::sync::Arc;

use magnus::typed_data::DataTypeBuilder;
use magnus::{
    function, memoize, Class, DataType, DataTypeFunctions, Module, Object, RClass, RModule,
    TryConvert, TypedData, Value,
};
use serde::{Deserialize, Serialize};
use tk::processors::bert::BertProcessing;
use tk::processors::byte_level::ByteLevel;
use tk::processors::roberta::RobertaProcessing;
use tk::processors::template::{SpecialToken, Template};
use tk::processors::PostProcessorWrapper;
use tk::{Encoding, PostProcessor};

use super::RbResult;

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
        if let Ok(v) = ob.try_convert::<(String, u32)>() {
            Ok(Self(v.into()))
        } else if let Ok(v) = ob.try_convert::<(u32, String)>() {
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
        if let Ok(s) = ob.try_convert::<String>() {
            Ok(Self(
                s.try_into().unwrap(), //.map_err(RbError::from)?,
            ))
        } else if let Ok(s) = ob.try_convert::<Vec<String>>() {
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
    ) ->  RbPostProcessor {
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
    fn class() -> RClass {
        *memoize!(RClass: {
          let class: RClass = crate::processors().const_get("PostProcessor").unwrap();
          class.undef_alloc_func();
          class
        })
    }

    fn data_type() -> &'static DataType {
        memoize!(DataType: DataTypeBuilder::<RbPostProcessor>::new("Tokenizers::Processors::PostProcessor").build())
    }

    fn class_for(value: &Self) -> RClass {
        match *value.processor {
            PostProcessorWrapper::Bert(_) => *memoize!(RClass: {
                let class: RClass = crate::processors().const_get("BertProcessing").unwrap();
                class.undef_alloc_func();
                class
            }),
            PostProcessorWrapper::ByteLevel(_) => *memoize!(RClass: {
                let class: RClass = crate::processors().const_get("ByteLevel").unwrap();
                class.undef_alloc_func();
                class
            }),
            PostProcessorWrapper::Roberta(_) => *memoize!(RClass: {
                let class: RClass = crate::processors().const_get("RobertaProcessing").unwrap();
                class.undef_alloc_func();
                class
            }),
            PostProcessorWrapper::Template(_) => *memoize!(RClass: {
                let class: RClass = crate::processors().const_get("TemplateProcessing").unwrap();
                class.undef_alloc_func();
                class
            }),
            _ => todo!(),
        }
    }
}

pub fn processors(module: &RModule) -> RbResult<()> {
    let post_processor = module.define_class("PostProcessor", Default::default())?;

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
