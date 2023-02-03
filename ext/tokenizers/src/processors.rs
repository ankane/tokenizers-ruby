use std::sync::Arc;

use magnus::typed_data::DataTypeBuilder;
use magnus::{
    function, memoize, Class, DataType, DataTypeFunctions, Module, Object, RClass, RModule,
    TryConvert, TypedData, Value,
};
use serde::{Deserialize, Serialize};
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

    let class = module.define_class("TemplateProcessing", post_processor)?;
    class.define_singleton_method("_new", function!(RbTemplateProcessing::new, 3))?;

    Ok(())
}
