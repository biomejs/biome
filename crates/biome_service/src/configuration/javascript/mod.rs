mod formatter;

use biome_deserialize::StringSet;
use biome_deserialize_macros::{Deserializable, Merge};
use bpaf::Bpaf;
pub use formatter::{javascript_formatter, JavascriptFormatter};
use serde::{Deserialize, Serialize};

/// A set of options applied to the JavaScript files
#[derive(
    Bpaf, Clone, Default, Debug, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JavascriptConfiguration {
    /// Formatting options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(javascript_formatter), optional)]
    pub formatter: Option<JavascriptFormatter>,

    /// Parsing options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(javascript_parser), optional)]
    pub parser: Option<JavascriptParser>,

    /// A list of global bindings that should be ignored by the analyzers
    ///
    /// If defined here, they should not emit diagnostics.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub globals: Option<StringSet>,
    //
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(javascript_organize_imports), optional)]
    pub organize_imports: Option<JavascriptOrganizeImports>,
}

impl JavascriptConfiguration {
    pub fn with_formatter() -> Self {
        Self {
            formatter: Some(JavascriptFormatter::default()),
            ..JavascriptConfiguration::default()
        }
    }
}

#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JavascriptOrganizeImports {}

/// Options that changes how the JavaScript parser behaves
#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JavascriptParser {
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// It enables the experimental and unsafe parsing of parameter decorators
    ///
    /// These decorators belong to an old proposal, and they are subject to change.
    pub unsafe_parameter_decorators_enabled: Option<bool>,
}
