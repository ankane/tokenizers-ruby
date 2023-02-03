use super::regex::regex;
use crate::RbResult;
use magnus::{exception, Error, TryConvert, Value};
use tk::normalizer::SplitDelimiterBehavior;
use tk::pattern::Pattern;

#[derive(Clone)]
pub enum RbPattern {
    Str(String),
    Regex(String), // TODO fix
}

impl TryConvert for RbPattern {
    fn try_convert(obj: Value) -> RbResult<Self> {
        if obj.is_kind_of(regex()) {
            todo!()
        } else {
            Ok(RbPattern::Str(obj.try_convert()?))
        }
    }
}

impl Pattern for RbPattern {
    fn find_matches(&self, inside: &str) -> tk::Result<Vec<(tk::Offsets, bool)>> {
        match self {
            RbPattern::Str(s) => {
                let mut chars = s.chars();
                if let (Some(c), None) = (chars.next(), chars.next()) {
                    c.find_matches(inside)
                } else {
                    s.find_matches(inside)
                }
            }
            RbPattern::Regex(_r) => {
                todo!()
            }
        }
    }
}

impl From<RbPattern> for tk::normalizers::replace::ReplacePattern {
    fn from(pattern: RbPattern) -> Self {
        match pattern {
            RbPattern::Str(s) => Self::String(s),
            RbPattern::Regex(_r) => todo!(),
        }
    }
}

impl From<RbPattern> for tk::pre_tokenizers::split::SplitPattern {
    fn from(pattern: RbPattern) -> Self {
        match pattern {
            RbPattern::Str(s) => Self::String(s),
            RbPattern::Regex(_r) => todo!(),
        }
    }
}

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
