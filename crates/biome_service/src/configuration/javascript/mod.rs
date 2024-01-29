mod formatter;

use biome_deserialize::StringSet;
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use bpaf::Bpaf;
pub use formatter::{
    partial_javascript_formatter, JavascriptFormatter, PartialJavascriptFormatter,
};
use serde::{Deserialize, Serialize};

/// A set of options applied to the JavaScript files
#[derive(Clone, Debug, Default, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(default, deny_unknown_fields))]
pub struct JavascriptConfiguration {
    /// Formatting options
    #[partial(type, bpaf(external(partial_javascript_formatter), optional))]
    pub formatter: JavascriptFormatter,

    /// Parsing options
    #[partial(type, bpaf(external(partial_javascript_parser), optional))]
    pub parser: JavascriptParser,

    /// A list of global bindings that should be ignored by the analyzers
    ///
    /// If defined here, they should not emit diagnostics.
    #[partial(bpaf(hide))]
    pub globals: StringSet,

    #[partial(type, bpaf(external(partial_javascript_organize_imports), optional))]
    pub organize_imports: JavascriptOrganizeImports,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(default, deny_unknown_fields))]
pub struct JavascriptOrganizeImports {}

/// Options that changes how the JavaScript parser behaves
#[derive(Clone, Debug, Default, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct JavascriptParser {
    /// It enables the experimental and unsafe parsing of parameter decorators
    ///
    /// These decorators belong to an old proposal, and they are subject to change.
    #[partial(bpaf(hide))]
    pub unsafe_parameter_decorators_enabled: bool,
}
