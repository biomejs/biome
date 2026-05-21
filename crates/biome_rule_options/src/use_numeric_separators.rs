use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NumericLiteralSeparatorOptions {
    /// Minimum number of digits required before adding separators.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub minimum_digits: Option<u8>,
    /// Number of digits between separators.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub group_length: Option<NonZeroU8>,
}

impl NumericLiteralSeparatorOptions {
    /// Returns the resolved options, using the provided defaults for any fields
    /// that are not set.
    pub fn resolve(
        &self,
        default_min_digits: u8,
        default_group_length: NonZeroU8,
    ) -> ResolvedOptions {
        ResolvedOptions {
            minimum_digits: self.minimum_digits.unwrap_or(default_min_digits),
            group_length: self.group_length.unwrap_or(default_group_length),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedOptions {
    pub minimum_digits: u8,
    pub group_length: NonZeroU8,
}

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseNumericSeparatorsOptions {
    /// Options for binary literals (e.g., `0b1010_0001`).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub binary: Option<NumericLiteralSeparatorOptions>,
    /// Options for octal literals (e.g., `0o1234_5670`).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub octal: Option<NumericLiteralSeparatorOptions>,
    /// Options for decimal literals (e.g., `1_234_567`).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub decimal: Option<NumericLiteralSeparatorOptions>,
    /// Options for hexadecimal literals (e.g., `0xAB_CD`).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub hexadecimal: Option<NumericLiteralSeparatorOptions>,
}

impl UseNumericSeparatorsOptions {
    pub const DEFAULT_BINARY_MIN_DIGITS: u8 = 0;
    pub const DEFAULT_BINARY_GROUP_LENGTH: NonZeroU8 = NonZeroU8::new(4).unwrap();
    pub const DEFAULT_OCTAL_MIN_DIGITS: u8 = 0;
    pub const DEFAULT_OCTAL_GROUP_LENGTH: NonZeroU8 = NonZeroU8::new(4).unwrap();
    pub const DEFAULT_DECIMAL_MIN_DIGITS: u8 = 5;
    pub const DEFAULT_DECIMAL_GROUP_LENGTH: NonZeroU8 = NonZeroU8::new(3).unwrap();
    pub const DEFAULT_HEXADECIMAL_MIN_DIGITS: u8 = 0;
    pub const DEFAULT_HEXADECIMAL_GROUP_LENGTH: NonZeroU8 = NonZeroU8::new(2).unwrap();

    /// Returns resolved options for binary literals.
    pub fn binary(&self) -> ResolvedOptions {
        match &self.binary {
            Some(opts) => opts.resolve(
                Self::DEFAULT_BINARY_MIN_DIGITS,
                Self::DEFAULT_BINARY_GROUP_LENGTH,
            ),
            None => ResolvedOptions {
                minimum_digits: Self::DEFAULT_BINARY_MIN_DIGITS,
                group_length: Self::DEFAULT_BINARY_GROUP_LENGTH,
            },
        }
    }

    /// Returns resolved options for octal literals.
    pub fn octal(&self) -> ResolvedOptions {
        match &self.octal {
            Some(opts) => opts.resolve(
                Self::DEFAULT_OCTAL_MIN_DIGITS,
                Self::DEFAULT_OCTAL_GROUP_LENGTH,
            ),
            None => ResolvedOptions {
                minimum_digits: Self::DEFAULT_OCTAL_MIN_DIGITS,
                group_length: Self::DEFAULT_OCTAL_GROUP_LENGTH,
            },
        }
    }

    /// Returns resolved options for decimal literals.
    pub fn decimal(&self) -> ResolvedOptions {
        match &self.decimal {
            Some(opts) => opts.resolve(
                Self::DEFAULT_DECIMAL_MIN_DIGITS,
                Self::DEFAULT_DECIMAL_GROUP_LENGTH,
            ),
            None => ResolvedOptions {
                minimum_digits: Self::DEFAULT_DECIMAL_MIN_DIGITS,
                group_length: Self::DEFAULT_DECIMAL_GROUP_LENGTH,
            },
        }
    }

    /// Returns resolved options for hexadecimal literals.
    pub fn hexadecimal(&self) -> ResolvedOptions {
        match &self.hexadecimal {
            Some(opts) => opts.resolve(
                Self::DEFAULT_HEXADECIMAL_MIN_DIGITS,
                Self::DEFAULT_HEXADECIMAL_GROUP_LENGTH,
            ),
            None => ResolvedOptions {
                minimum_digits: Self::DEFAULT_HEXADECIMAL_MIN_DIGITS,
                group_length: Self::DEFAULT_HEXADECIMAL_GROUP_LENGTH,
            },
        }
    }
}
