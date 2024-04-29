use crate::{GritPattern, ParseError};
use grit_pattern_matcher::pattern::Pattern;

pub fn parse_pattern(source: String) -> Result<GritPattern, ParseError> {
    Ok(GritPattern {
        _pattern: Pattern::Undefined,

        source,
    })
}
