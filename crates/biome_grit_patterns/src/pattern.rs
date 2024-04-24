use crate::grit_context::GritQueryContext;
use grit_core_patterns::pattern::patterns::Pattern;

pub struct GritPattern {
    pub(crate) _pattern: Pattern<GritQueryContext>,

    pub source: String,
}
