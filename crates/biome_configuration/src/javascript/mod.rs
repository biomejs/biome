mod formatter;
use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
#[cfg(feature = "cli")]
use bpaf::Bpaf;
pub use formatter::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub type ExperimentalEmbeddedSnippetsEnabled = Bool<false>;
pub type ExperimentalPnpmCatalogsEnabled = Bool<false>;

/// Options applied to JavaScript, TypeScript, JSX, TSX, and supported languages that embed
/// JavaScript.
///
/// Language-specific settings take precedence over corresponding global settings. Global settings
/// apply when their language-specific counterparts are omitted, unless stated otherwise.
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsConfiguration {
    /// JavaScript parser options.
    #[cfg_attr(feature = "cli", bpaf(external(js_parser_configuration), optional))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser: Option<JsParserConfiguration>,

    /// JavaScript formatter options.
    #[cfg_attr(feature = "cli", bpaf(external(js_formatter_configuration), optional))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<JsFormatterConfiguration>,

    /// JavaScript linter options.
    #[cfg_attr(feature = "cli", bpaf(external(js_linter_configuration), optional))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<JsLinterConfiguration>,

    /// JavaScript assist options.
    #[cfg_attr(feature = "cli", bpaf(external(js_assist_configuration), optional))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<JsAssistConfiguration>,

    /// JavaScript module and dependency resolver options.
    #[cfg_attr(feature = "cli", bpaf(external(js_resolver_configuration), optional))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<JsResolverConfiguration>,

    /// A list of additional names that Biome's JavaScript linter treats as predefined global
    /// bindings.
    ///
    /// Use this for globals supplied by a runtime, framework, or external script that are not
    /// declared in the source file.
    #[cfg_attr(feature = "cli", bpaf(pure(Default::default()), hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub globals: Option<rustc_hash::FxHashSet<Box<str>>>,

    /// Configures how the analyzer accounts for imports used by JSX. This option does not transform
    /// JSX or select a runtime for a build tool. Defaults to `transparent`.
    #[cfg_attr(feature = "cli", bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx_runtime: Option<JsxRuntime>,

    /// Enables experimental parsing, formatting, linting, diagnostics, and fixes for CSS and
    /// GraphQL snippets embedded in JavaScript and TypeScript template literals.
    ///
    /// Biome recognizes CSS in `css` and `styled` templates and GraphQL in `gql` and `graphql`
    /// templates or calls. Templates containing interpolations are not currently supported.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(hide))]
    pub experimental_embedded_snippets_enabled: Option<ExperimentalEmbeddedSnippetsEnabled>,
}

/// Resolver options specific to JavaScript files
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsResolverConfiguration {
    /// Enables pnpm workspace catalog resolution for JavaScript package manifests.
    ///
    /// Opt-in:
    /// - Set `javascript.resolver.experimentalPnpmCatalogs` to `true`.
    ///
    /// Scope:
    /// - Resolves `catalog:` and `catalog:<name>` dependency versions from
    ///   `package.json`.
    /// - Applies to `dependencies`, `devDependencies`, and `peerDependencies`.
    ///
    /// Fail-safe behavior:
    /// - If `pnpm-workspace.yaml` is missing, unreadable, or cannot be parsed,
    ///   Biome silently falls back to the default behavior (as if this option
    ///   were disabled).
    /// - Unknown keys and unsupported value shapes in `pnpm-workspace.yaml` are
    ///   ignored.
    ///
    /// Limitations:
    /// - Only `pnpm-workspace.yaml` is read.
    /// - Biome only reads top-level `catalog` / `catalogs` mappings and scalar
    ///   string entries.
    ///
    /// Defaults to `false`.
    #[cfg_attr(feature = "cli", bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental_pnpm_catalogs: Option<ExperimentalPnpmCatalogsEnabled>,
}

pub type UnsafeParameterDecoratorsEnabled = Bool<false>;
pub type JsxEverywhere = Bool<true>;
pub type JsGritMetavariable = Bool<false>;

/// Options that change how the JavaScript parser behaves.
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsParserConfiguration {
    /// Enables parsing decorators on class parameters. This syntax belongs to an old experimental
    /// proposal and may change. Defaults to `false`.
    #[cfg_attr(feature = "cli", bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsafe_parameter_decorators_enabled: Option<UnsafeParameterDecoratorsEnabled>,

    /// Enables parsing Grit metavariables in JavaScript and TypeScript syntax. Defaults to `false`.
    #[cfg_attr(feature = "cli", bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grit_metavariables: Option<JsGritMetavariable>,

    /// Controls whether `.js`, `.mjs`, and `.cjs` files may contain JSX syntax. Disabling this
    /// option causes JSX in those files to raise a diagnostic. Defaults to `true`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("jsx-everywhere"), argument("true|false"), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx_everywhere: Option<JsxEverywhere>,
}

/// How Biome's analyzer accounts for imports used by JSX.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum JsxRuntime {
    /// An automatic or native JSX environment that doesn't require an in-scope factory import.
    #[default]
    Transparent,

    /// A classic React environment that requires the React import, or custom JSX factory imports
    /// configured in `tsconfig.json`.
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
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsLinterConfiguration {
    /// Enables or disables the linter for JavaScript and languages that extend it.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("javascript-linter-enabled"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsLinterEnabled>,
}

pub type JsAssistEnabled = Bool<true>;

/// Assist options specific to the JavaScript assist
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsAssistConfiguration {
    /// Enables or disables assist actions for JavaScript and languages that extend it.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("javascript-assist-enabled"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsAssistEnabled>,
}

impl JsLinterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}
