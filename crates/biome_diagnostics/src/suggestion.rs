use ::serde::{Deserialize, Serialize};
use biome_console::MarkupBuf;
use biome_rowan::TextRange;
use biome_text_edit::TextEdit;

/// Indicates how a tool should manage this suggestion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum Applicability {
    /// The suggestion is definitely what the user intended.
    /// This suggestion should be automatically applied.
    Always,
    /// The suggestion may be what the user intended, but it is uncertain.
    /// The suggestion should result in valid JavaScript/TypeScript code if it is applied.
    MaybeIncorrect,
}

/// A Suggestion that is provided by Biome's linter, and
/// can be reported to the user, and can be automatically
/// applied if it has the right [`Applicability`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CodeSuggestion {
    pub span: TextRange,
    pub applicability: Applicability,
    pub msg: MarkupBuf,
    pub suggestion: TextEdit,
    pub labels: Vec<TextRange>,
}
