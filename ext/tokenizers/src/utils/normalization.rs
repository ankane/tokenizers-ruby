use crate::RbResult;
use magnus::{exception, Error, TryConvert, Value};
use tk::normalizer::SplitDelimiterBehavior;

#[derive(Clone)]
pub struct RbSplitDelimiterBehavior(pub SplitDelimiterBehavior);

impl TryConvert for RbSplitDelimiterBehavior {
    fn try_convert(obj: Value) -> RbResult<Self> {
        let s = obj.try_convert::<String>()?;

        Ok(Self(match s.as_str() {
            "removed" => Ok(SplitDelimiterBehavior::Removed),
            "isolated" => Ok(SplitDelimiterBehavior::Isolated),
            "merged_with_previous" => Ok(SplitDelimiterBehavior::MergedWithPrevious),
            "merged_with_next" => Ok(SplitDelimiterBehavior::MergedWithNext),
            "contiguous" => Ok(SplitDelimiterBehavior::Contiguous),
            _ => Err(Error::new(
                exception::arg_error(),
                "Wrong value for SplitDelimiterBehavior, expected one of: \
                `removed, isolated, merged_with_previous, merged_with_next, contiguous`",
            )),
        }?))
    }
}

impl From<RbSplitDelimiterBehavior> for SplitDelimiterBehavior {
    fn from(v: RbSplitDelimiterBehavior) -> Self {
        v.0
    }
}
