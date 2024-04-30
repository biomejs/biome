use crate::grit_context::GritQueryContext;
use grit_pattern_matcher::pattern::Pattern;

pub struct GritPattern {
    pub(crate) _pattern: Pattern<GritQueryContext>,

    pub source: String,
}
