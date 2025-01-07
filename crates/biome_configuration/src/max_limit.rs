use biome_deserialize::{Deserializable, DeserializableValue, DeserializationContext};
use biome_deserialize_macros::Merge;
use std::{
    fmt,
    num::{NonZeroU64, ParseIntError},
    str::FromStr,
};

/// Limit the size of files to 1.0 MiB by default
pub const DEFAULT_FILE_SIZE_LIMIT: NonZeroU64 =
    // SAFETY: This constant is initialized with a non-zero value
    unsafe { NonZeroU64::new_unchecked(1024 * 1024) };

#[derive(Clone, Copy, Eq, Merge, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MaxLimit(pub NonZeroU64);

impl Default for MaxLimit {
    fn default() -> Self {
        Self(DEFAULT_FILE_SIZE_LIMIT)
    }
}

impl FromStr for MaxLimit {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(NonZeroU64::from_str(s)?))
    }
}

impl Deserializable for MaxLimit {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        NonZeroU64::deserialize(ctx, value, name).map(Self)
    }
}

impl fmt::Debug for MaxLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<NonZeroU64> for MaxLimit {
    fn from(value: NonZeroU64) -> Self {
        Self(value)
    }
}

impl From<MaxLimit> for NonZeroU64 {
    fn from(value: MaxLimit) -> Self {
        value.0
    }
}

impl From<MaxLimit> for usize {
    fn from(value: MaxLimit) -> Self {
        Self::try_from(NonZeroU64::from(value).get()).unwrap_or(Self::MAX)
    }
}
