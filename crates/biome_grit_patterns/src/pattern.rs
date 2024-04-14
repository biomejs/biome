#[cfg(feature = "grit")]
use marzano_core::pattern::patterns::Pattern;

pub struct GritPattern {
    #[allow(dead_code)]
    #[cfg(feature = "grit")]
    pub(crate) pattern: Pattern,

    pub source: String,
}
