//! A simple implementation of feature flags.

/// Returns `true` if this is an unstable build of Rome
pub const fn is_unstable() -> bool {
    option_env!("BIOME_VERSION").is_none()
}
