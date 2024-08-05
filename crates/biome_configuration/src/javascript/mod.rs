mod formatter;

use std::str::FromStr;

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
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct JavascriptConfiguration {
    /// Formatting options
    #[partial(type, bpaf(external(partial_javascript_formatter), optional))]
    pub formatter: JavascriptFormatter,

    /// Linter options
    #[partial(type, bpaf(external(partial_javascript_linter), optional))]
    pub linter: JavascriptLinter,

    /// Assists options
    #[partial(type, bpaf(external(partial_javascript_assists), optional))]
    pub assists: JavascriptAssists,

    /// Parsing options
    #[partial(type, bpaf(external(partial_javascript_parser), optional))]
    pub parser: JavascriptParser,

    /// A list of global bindings that should be ignored by the analyzers
    ///
    /// If defined here, they should not emit diagnostics.
    #[partial(bpaf(hide))]
    pub globals: StringSet,

    /// Indicates the type of runtime or transformation used for interpreting JSX.
    #[partial(bpaf(hide))]
    pub jsx_runtime: JsxRuntime,

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

/// Indicates the type of runtime or transformation used for interpreting JSX.
#[derive(
    Bpaf, Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum JsxRuntime {
    /// Indicates a modern or native JSX environment, that doesn't require
    /// special handling by Biome.
    #[default]
    Transparent,

    /// Indicates a classic React environment that requires the `React` import.
    ///
    /// Corresponds to the `react` value for the `jsx` option in TypeScript's
    /// `tsconfig.json`.
    ///
    /// This option should only be necessary if you cannot upgrade to a React
    /// version that supports the new JSX runtime. For more information about
    /// the old vs. new JSX runtime, please see:
    /// <https://legacy.reactjs.org/blog/2020/09/22/introducing-the-new-jsx-transform.html>
    ReactClassic,
}

impl FromStr for JsxRuntime {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "transparent" => Ok(Self::Transparent),
            "react-classic" | "reactClassic" => Ok(Self::ReactClassic),
            _ => Err("Unexpected value".to_string()),
        }
    }
}

/// Linter options specific to the JavaScript linter
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct JavascriptLinter {
    /// Control the linter for JavaScript (and its super languages) files.
    #[partial(bpaf(long("javascript-linter-enabled"), argument("true|false"), optional))]
    pub enabled: bool,
}

impl Default for JavascriptLinter {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl PartialJavascriptLinter {
    pub fn get_linter_configuration(&self) -> JavascriptLinter {
        JavascriptLinter {
            enabled: self.enabled.unwrap_or_default(),
        }
    }
}

/// Linter options specific to the JavaScript linter
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct JavascriptAssists {
    /// Control the linter for JavaScript (and its super languages) files.
    #[partial(bpaf(long("javascript-assists-enabled"), argument("true|false"), optional))]
    pub enabled: bool,
}

impl Default for JavascriptAssists {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl PartialJavascriptAssists {
    pub fn get_linter_configuration(&self) -> JavascriptAssists {
        JavascriptAssists {
            enabled: self.enabled.unwrap_or_default(),
        }
    }
}
