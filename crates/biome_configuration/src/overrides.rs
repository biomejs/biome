use crate::analyzer::assist::AssistEnabled;
use crate::analyzer::{LinterEnabled, RuleDomains};
use crate::formatter::{FormatWithErrorsEnabled, FormatterEnabled};
#[cfg(feature = "lang_html")]
use crate::html::HtmlConfiguration;
use crate::max_size::MaxSize;
use crate::{GritConfiguration, Rules};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, DelimiterSpacing, Expand, IndentStyle,
    IndentWidth, LineEnding, LineWidth, TrailingNewline,
};
#[cfg(feature = "lang_js")]
use biome_js_formatter::context::trailing_commas::TrailingCommas;
#[cfg(feature = "plugins")]
use biome_plugin_loader::Plugins;
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Configures settings for files selected by each override. When multiple overrides configure the
/// same single-value setting, the last matching override takes precedence.
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Overrides(pub Vec<OverridePattern>);

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverridePattern {
    /// A list of glob patterns selecting the files to which this override applies. If omitted, the
    /// override applies to every file. An empty list applies to no files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<OverrideGlobs>,

    /// JavaScript-specific settings for matched files.
    #[cfg_attr(feature = "lang_js", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg(feature = "lang_js")]
    pub javascript: Option<crate::JsConfiguration>,

    /// JSON-specific settings for matched files.
    #[cfg(feature = "lang_json")]
    #[cfg_attr(feature = "lang_json", serde(skip_serializing_if = "Option::is_none"))]
    pub json: Option<crate::JsonConfiguration>,

    /// CSS-specific settings for matched files.
    #[cfg_attr(feature = "lang_css", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg(feature = "lang_css")]
    pub css: Option<crate::CssConfiguration>,

    /// GraphQL-specific settings for matched files.
    #[cfg(feature = "lang_graphql")]
    #[cfg_attr(
        feature = "lang_graphql",
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub graphql: Option<crate::graphql::GraphqlConfiguration>,

    /// GritQL-specific settings for matched files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grit: Option<GritConfiguration>,

    /// HTML-specific settings for matched files.
    #[cfg(feature = "lang_html")]
    #[cfg_attr(feature = "lang_html", serde(skip_serializing_if = "Option::is_none"))]
    pub html: Option<HtmlConfiguration>,

    /// Formatter settings for matched files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<OverrideFormatterConfiguration>,

    /// Linter settings for matched files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<OverrideLinterConfiguration>,

    /// Assist settings for matched files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<OverrideAssistConfiguration>,

    /// File-handling settings for matched files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<OverrideFilesConfiguration>,

    /// Additional plugins for matched files.
    #[cfg(feature = "plugins")]
    #[cfg_attr(feature = "plugins", serde(skip_serializing_if = "Option::is_none"))]
    pub plugins: Option<Plugins>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum OverrideGlobs {
    Globs(Box<[biome_glob::NormalizedGlob]>),
    EditorconfigGlob(Box<biome_glob::editorconfig::EditorconfigGlob>),
}
impl OverrideGlobs {
    /// Normalize `path` and match it against the list of globs.
    pub fn is_match_candidate(&self, path: &biome_glob::CandidatePath) -> bool {
        match self {
            Self::Globs(globs) => path.matches_with_exceptions(globs),
            Self::EditorconfigGlob(glob) => glob.is_match_candidate(path),
        }
    }
}
impl biome_deserialize::Deserializable for OverrideGlobs {
    fn deserialize(
        ctx: &mut dyn biome_deserialize::DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        biome_deserialize::Deserializable::deserialize(ctx, value, name).map(OverrideGlobs::Globs)
    }
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for OverrideGlobs {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("OverrideGlobs")
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        Vec::<biome_glob::Glob>::json_schema(generator)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideFormatterConfiguration {
    /// Enables or disables the formatter for matched files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(hide))]
    pub enabled: Option<FormatterEnabled>,

    /// Allows formatting matched files that contain syntax errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(hide))]
    pub format_with_errors: Option<FormatWithErrorsEnabled>,

    /// Uses tabs or spaces for indentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(long("indent-style"), argument("tab|space")))]
    pub indent_style: Option<IndentStyle>,

    /// The indentation width. Deprecated, use `indentWidth` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deserializable(deprecated(use_instead = "formatter.indentWidth"))]
    #[cfg_attr(feature = "cli", bpaf(long("indent-size"), argument("NUMBER")))]
    pub indent_size: Option<IndentWidth>,

    /// Sets the indentation width. With space indentation, this is the number of spaces emitted per
    /// indentation level. With tab indentation, Biome emits one tab per level and uses this value as
    /// the tab's display width when calculating line length. Accepted values are `0` through `24`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(long("indent-width"), argument("NUMBER")))]
    pub indent_width: Option<IndentWidth>,

    /// The line ending.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("line-ending"), argument("lf|crlf|cr|auto"))
    )]
    pub line_ending: Option<LineEnding>,

    /// Sets the preferred maximum line width used when deciding where to wrap code. Some content,
    /// such as long unbreakable strings, may still exceed this width. Accepted values are `1` through
    /// `320`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(long("line-width"), argument("NUMBER")))]
    pub line_width: Option<LineWidth>,

    /// The attribute position style in HTML-like languages.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("attribute-position"), argument("multiline|auto"))
    )]
    pub attribute_position: Option<AttributePosition>,

    /// Controls the placement of the closing bracket for multiline HTML and JSX opening tags. Biome
    /// places the bracket at the end of the last attribute line when enabled and on its own line when
    /// disabled. This also affects self-closing HTML elements, but self-closing JSX elements are
    /// unaffected.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("bracket-same-line"), argument("true|false"))
    )]
    pub bracket_same_line: Option<BracketSameLine>,

    /// Controls spaces inside braces in supported single-line structures. The affected structures
    /// vary by language.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(long("bracket-spacing"), argument("true|false")))]
    pub bracket_spacing: Option<BracketSpacing>,

    /// Controls spaces immediately inside supported delimiters when their content fits on one line.
    /// It doesn't add spaces before opening delimiters or inside empty delimiters.
    ///
    /// The affected delimiters vary by language. If unset, uses the configured formatter setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("delimiter-spacing"), argument("true|false"))
    )]
    pub delimiter_spacing: Option<DelimiterSpacing>,

    /// Controls whether arrays and objects are formatted on one line or multiple lines.
    ///
    /// `auto` formats objects on multiple lines if the first property has a newline, and arrays on
    /// one line if they fit.
    ///
    /// `always` formats arrays and objects on multiple lines.
    ///
    /// `never` formats arrays and objects on one line if they fit.
    ///
    /// If unset, uses the configured formatter setting.
    ///
    /// When formatting `package.json`, Biome uses `always` unless configured otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("object-wrap"), argument("auto|always|never"))
    )]
    pub expand: Option<Expand>,

    /// Prints trailing commas wherever possible in multiline comma-separated structures. This is a
    /// legacy override option. Prefer `javascript.formatter.trailingCommas`.
    #[cfg(feature = "lang_js")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("trailing-commas"), argument("all|es5|none"))
    )]
    pub trailing_commas: Option<TrailingCommas>,

    /// Whether to add a trailing newline at the end of matched files. Disabling this option can
    /// cause compatibility problems with other tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("trailing-newline"), argument("true|false"))
    )]
    pub trailing_newline: Option<TrailingNewline>,
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideLinterConfiguration {
    /// Enables or disables the linter for matched files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(hide))]
    pub enabled: Option<LinterEnabled>,

    /// The lint-rule configuration for matched files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(pure(Default::default()), hide))]
    pub rules: Option<Rules>,

    /// The lint-domain configuration for matched files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(pure(Default::default()), optional, hide))]
    pub domains: Option<RuleDomains>,
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OverrideFilesConfiguration {
    /// The maximum source-file size in bytes for matched files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<MaxSize>,
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideAssistConfiguration {
    /// Enables or disables assist actions for matched files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(hide))]
    pub enabled: Option<AssistEnabled>,

    /// The assist-action configuration for matched files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(pure(crate::analyzer::assist::Actions::default()), optional, hide)
    )]
    pub actions: Option<crate::analyzer::assist::Actions>,
}
