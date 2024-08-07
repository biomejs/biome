//! A simple implementation of feature flags.

use std::ops::Deref;
use std::sync::LazyLock;

/// Returns `true` if this is an unstable build of Biome
pub fn is_unstable() -> bool {
    BIOME_VERSION.deref().is_none()
}

/// The internal version of Biome. This is usually supplied during the CI build
pub static BIOME_VERSION: LazyLock<Option<&str>> = LazyLock::new(|| option_env!("BIOME_VERSION"));
