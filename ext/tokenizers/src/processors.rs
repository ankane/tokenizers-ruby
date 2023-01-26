use std::sync::Arc;

use magnus::typed_data::DataTypeBuilder;
use magnus::{memoize, Class, DataType, DataTypeFunctions, Module, RClass, TypedData};
use serde::{Deserialize, Serialize};
use tk::processors::PostProcessorWrapper;
use tk::{Encoding, PostProcessor};

use super::module;

#[derive(DataTypeFunctions, Clone, Deserialize, Serialize)]
pub struct RbPostProcessor {
    #[serde(flatten)]
    pub processor: Arc<PostProcessorWrapper>,
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

unsafe impl TypedData for RbPostProcessor {
    fn class() -> RClass {
        *memoize!(RClass: {
          let class: RClass = module().const_get("PostProcessor").unwrap();
          class.undef_alloc_func();
          class
        })
    }

    fn data_type() -> &'static DataType {
        memoize!(DataType: DataTypeBuilder::<RbPostProcessor>::new("Tokenizers::PostProcessor").build())
    }

    fn class_for(value: &Self) -> RClass {
        match &value.processor {
            _ => Self::class(),
        }
    }
}
