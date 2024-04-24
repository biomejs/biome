use crate::{GritPattern, ParseError};
use grit_core_patterns::pattern::patterns::Pattern;

pub fn parse_pattern(source: String) -> Result<GritPattern, ParseError> {
    Ok(GritPattern {
        _pattern: Pattern::Undefined,

        source,
    })
}
