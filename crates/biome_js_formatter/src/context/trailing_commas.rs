use crate::prelude::*;
use crate::{JsFormatContext, JsFormatOptions};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::prelude::{if_group_breaks, text};
use biome_formatter::write;
use biome_formatter::{Format, FormatResult};
use std::fmt;
use std::str::FromStr;

/// This enum is used within formatting functions to print or omit trailing commas.
#[derive(Debug, Copy, Clone)]
pub(crate) enum FormatTrailingCommas {
    /// Print trailing commas if the option is [TrailingCommas::All].
    All,
    /// Print trailing commas if the option is [TrailingCommas::All] or [TrailingCommas::Es5].
    ES5,
}

impl FormatTrailingCommas {
    /// This function returns corresponding [TrailingSeparator] for [format_separated] function.
    pub fn trailing_separator(&self, options: &JsFormatOptions) -> TrailingSeparator {
        if options.trailing_commas.is_none() {
            return TrailingSeparator::Omit;
        }

        match self {
            FormatTrailingCommas::All => {
                if options.trailing_commas.is_all() {
                    TrailingSeparator::Allowed
                } else {
                    TrailingSeparator::Omit
                }
            }
            FormatTrailingCommas::ES5 => TrailingSeparator::Allowed,
        }
    }
}

impl Format<JsFormatContext> for FormatTrailingCommas {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        if f.options().trailing_commas.is_none() {
            return Ok(());
        }

        if matches!(self, FormatTrailingCommas::ES5) || f.options().trailing_commas().is_all() {
            write!(f, [if_group_breaks(&text(","))])?
        }

        Ok(())
    }
}

/// Print trailing commas wherever possible in multi-line comma-separated syntactic structures.
#[derive(Clone, Copy, Default, Debug, Deserializable, Eq, Hash, Merge, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum TrailingCommas {
    /// Trailing commas wherever possible (including function parameters and calls).
    #[default]
    All,
    /// Trailing commas where valid in ES5 (objects, arrays, etc.). No trailing commas in type parameters in TypeScript.
    Es5,
    /// No trailing commas.
    None,
}

impl TrailingCommas {
    pub const fn is_es5(&self) -> bool {
        matches!(self, TrailingCommas::Es5)
    }
    pub const fn is_all(&self) -> bool {
        matches!(self, TrailingCommas::All)
    }
    pub const fn is_none(&self) -> bool {
        matches!(self, TrailingCommas::None)
    }
}

impl FromStr for TrailingCommas {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "es5" => Ok(Self::Es5),
            "all" => Ok(Self::All),
            "none" => Ok(Self::None),
            // TODO: replace this error with a diagnostic
            _ => Err("Value not supported for TrailingCommas"),
        }
    }
}

impl fmt::Display for TrailingCommas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrailingCommas::Es5 => std::write!(f, "ES5"),
            TrailingCommas::All => std::write!(f, "All"),
            TrailingCommas::None => std::write!(f, "None"),
        }
    }
}
