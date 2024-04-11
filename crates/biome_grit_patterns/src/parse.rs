use crate::{GritPattern, ParseError};
#[cfg(feature = "grit")]
use marzano_core::pattern::patterns::Pattern;

pub fn parse_pattern(source: String) -> Result<GritPattern, ParseError> {
    Ok(GritPattern {
        // FIXME: Do some real parsing here.
        #[cfg(feature = "grit")]
        pattern: Pattern::Undefined,

        source,
    })
}
