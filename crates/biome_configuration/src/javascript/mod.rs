mod formatter;
use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use bpaf::Bpaf;
pub use formatter::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// A set of options applied to the JavaScript files
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsConfiguration {
    /// Parsing options
    #[bpaf(external(js_parser_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser: Option<JsParserConfiguration>,

    /// Formatting options
    #[bpaf(external(js_formatter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<JsFormatterConfiguration>,

    /// Linter options
    #[bpaf(external(js_linter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<JsLinterConfiguration>,

    /// Assist options
    #[bpaf(external(js_assist_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<JsAssistConfiguration>,

    /// A list of global bindings that should be ignored by the analyzers
    ///
    /// If defined here, they should not emit diagnostics.
    #[bpaf(pure(Default::default()), hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub globals: Option<rustc_hash::FxHashSet<Box<str>>>,

    /// Indicates the type of runtime or transformation used for interpreting JSX.
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx_runtime: Option<JsxRuntime>,
}

pub type UnsafeParameterDecoratorsEnabled = Bool<false>;
pub type JsxEverywhere = Bool<true>;
pub type JsGritMetavariable = Bool<false>;

/// Options that changes how the JavaScript parser behaves
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsParserConfiguration {
    /// It enables the experimental and unsafe parsing of parameter decorators
    ///
    /// These decorators belong to an old proposal, and they are subject to change.
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsafe_parameter_decorators_enabled: Option<UnsafeParameterDecoratorsEnabled>,

    /// Enables parsing of Grit metavariables.
    /// Defaults to `false`.
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grit_metavariables: Option<JsGritMetavariable>,

    /// When enabled, files like `.js`/`.ts` can contain JSX syntax. Defaults to `true`.
    #[bpaf(long("jsx-everywhere"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx_everywhere: Option<JsxEverywhere>,
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

pub type JsLinterEnabled = Bool<true>;

/// Linter options specific to the JavaScript linter
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsLinterConfiguration {
    /// Control the linter for JavaScript (and its super languages) files.
    #[bpaf(long("javascript-linter-enabled"), argument("true|false"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsLinterEnabled>,
}

pub type JsAssistEnabled = Bool<true>;

/// Assist options specific to the JavaScript assist
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsAssistConfiguration {
    /// Control the assist for JavaScript (and its super languages) files.
    #[bpaf(long("javascript-assist-enabled"), argument("true|false"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsAssistEnabled>,
}

impl JsLinterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}
