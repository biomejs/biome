use crate::prelude::*;
use crate::{JsFormatContext, JsFormatOptions};
use biome_deserialize::json::with_only_known_variants;
use biome_deserialize::{DeserializationDiagnostic, VisitNode};
use biome_formatter::formatter::Formatter;
use biome_formatter::prelude::{if_group_breaks, text};
use biome_formatter::write;
use biome_formatter::{Format, FormatResult};
use biome_json_syntax::JsonLanguage;
use biome_rowan::SyntaxNode;
use std::fmt;
use std::str::FromStr;

/// This enum is used within formatting functions to print or omit trailing comma.
#[derive(Debug, Copy, Clone)]
pub(crate) enum FormatTrailingComma {
    /// Print trailing comma if the option is [TrailingComma::All].
    All,
    /// Print trailing comma if the option is [TrailingComma::All] or [TrailingComma::Es5].
    ES5,
}

impl FormatTrailingComma {
    /// This function returns corresponding [TrailingSeparator] for [format_separated] function.
    pub fn trailing_separator(&self, options: &JsFormatOptions) -> TrailingSeparator {
        if options.trailing_comma.is_none() {
            return TrailingSeparator::Omit;
        }

        match self {
            FormatTrailingComma::All => {
                if options.trailing_comma.is_all() {
                    TrailingSeparator::Allowed
                } else {
                    TrailingSeparator::Omit
                }
            }
            FormatTrailingComma::ES5 => TrailingSeparator::Allowed,
        }
    }
}

impl Format<JsFormatContext> for FormatTrailingComma {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        if f.options().trailing_comma.is_none() {
            return Ok(());
        }

        if matches!(self, FormatTrailingComma::ES5) || f.options().trailing_comma().is_all() {
            write!(f, [if_group_breaks(&text(","))])?
        }

        Ok(())
    }
}

/// Print trailing commas wherever possible in multi-line comma-separated syntactic structures.
#[derive(Default, Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema),
    serde(rename_all = "camelCase")
)]
pub enum TrailingComma {
    /// Trailing commas wherever possible (including function parameters and calls).
    #[default]
    All,
    /// Trailing commas where valid in ES5 (objects, arrays, etc.). No trailing commas in type parameters in TypeScript.
    Es5,
    /// No trailing commas.
    None,
}

impl TrailingComma {
    pub(crate) const KNOWN_VALUES: &'static [&'static str] = &["all", "es5", "none"];

    pub const fn is_es5(&self) -> bool {
        matches!(self, TrailingComma::Es5)
    }
    pub const fn is_all(&self) -> bool {
        matches!(self, TrailingComma::All)
    }
    pub const fn is_none(&self) -> bool {
        matches!(self, TrailingComma::None)
    }
}

impl FromStr for TrailingComma {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "es5" | "ES5" => Ok(Self::Es5),
            "all" | "All" => Ok(Self::All),
            "none" | "None" => Ok(Self::None),
            // TODO: replace this error with a diagnostic
            _ => Err("Value not supported for TrailingComma"),
        }
    }
}

impl fmt::Display for TrailingComma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrailingComma::Es5 => std::write!(f, "ES5"),
            TrailingComma::All => std::write!(f, "All"),
            TrailingComma::None => std::write!(f, "None"),
        }
    }
}

impl VisitNode<JsonLanguage> for TrailingComma {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = with_only_known_variants(node, TrailingComma::KNOWN_VALUES, diagnostics)?;
        match node.inner_string_text().ok()?.text() {
            "all" => {
                *self = TrailingComma::All;
            }
            "es5" => {
                *self = TrailingComma::Es5;
            }
            "none" => {
                *self = TrailingComma::None;
            }
            _ => {}
        }
        Some(())
    }
}
