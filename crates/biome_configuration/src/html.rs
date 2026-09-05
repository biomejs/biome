use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    AttributePosition, BracketSameLine, IndentStyle, IndentWidth, LineEnding, LineWidth,
    TrailingNewline,
};
use biome_html_formatter::context::{
    IndentScriptAndStyle, SelfCloseVoidElements, WhitespaceSensitivity,
};
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

pub type ExperimentalFullSupportEnabled = Bool<false>;

/// Options applied to HTML and languages that extend it.
///
/// Full HTML support and the HTML formatter are experimental. Biome aims to minimize breaking
/// changes, but bug fixes and new features may change formatting output or diagnostics.
///
/// Language-specific settings take precedence over corresponding global settings. Global settings
/// apply when their language-specific counterparts are omitted, unless stated otherwise.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct HtmlConfiguration {
    /// Enables Biome's experimental full support for `.html`, `.vue`, `.svelte`, and `.astro` files.
    /// In this mode, Biome parses the complete document and can analyze or format its markup and
    /// supported embedded languages.
    ///
    /// When disabled, `.vue`, `.svelte`, and `.astro` files use legacy handling, which extracts their
    /// JavaScript or TypeScript portions and leaves the rest unchanged. This option selects how
    /// these files are processed. It does not enable the HTML formatter. Set
    /// `html.formatter.enabled` to `true` to format complete HTML, Vue, Svelte, and Astro files.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(hide))]
    pub experimental_full_support_enabled: Option<ExperimentalFullSupportEnabled>,

    /// HTML parsing options.
    #[cfg_attr(feature = "cli", bpaf(hide, pure(Default::default())))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser: Option<HtmlParserConfiguration>,

    /// HTML formatter options.
    #[cfg_attr(
        feature = "cli",
        bpaf(external(html_formatter_configuration), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<HtmlFormatterConfiguration>,

    /// HTML linter options.
    #[cfg_attr(feature = "cli", bpaf(external(html_linter_configuration), optional))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<HtmlLinterConfiguration>,

    /// HTML assist options.
    #[cfg_attr(feature = "cli", bpaf(external(html_assist_configuration), optional))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<HtmlAssistConfiguration>,
}

pub type HtmlFormatterEnabled = Bool<false>; // Keep it disabled by default while experimental.
pub type HtmlLinterEnabled = Bool<true>;
pub type HtmlAssistEnabled = Bool<true>;
pub type HtmlParseInterpolation = Bool<false>;
pub type HtmlParseVue = Bool<false>;

/// Options that change how the HTML parser behaves.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HtmlParserConfiguration {
    /// Enables double-curly interpolation expressions such as `{{ expression }}` in `.html` files.
    /// Defaults to `false`.
    pub interpolation: Option<HtmlParseInterpolation>,

    /// Enables parsing Vue syntax (`v-if`, `v-bind`, etc.) in `.html` files. Enabling this option
    /// also enables `interpolation` implicitly.
    ///
    /// Biome will already automatically enable Vue parsing in `.vue` files, so you probably don't need
    /// to enable this option. This only affects `.html` files, and does not change how `.vue`, `.svelte`,
    /// or `.astro` files are parsed. Defaults to `false`.
    pub vue: Option<HtmlParseVue>,
}

/// Options that change how the HTML formatter behaves.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct HtmlFormatterConfiguration {
    /// Enables or disables the formatter for HTML and languages that extend it. The formatter is
    /// experimental and disabled by default. Formatting complete HTML, Vue, Svelte, and Astro files
    /// requires `html.experimentalFullSupportEnabled` to be `true`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("html-formatter-enabled"), argument("true|false"), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<HtmlFormatterEnabled>,

    /// The indent style applied to HTML and languages that extend it. If unset, inherits the global
    /// indentation style.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("html-formatter-indent-style"), argument("tab|space"), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The indentation width applied to HTML and languages that extend it. If unset, inherits the
    /// global indentation width.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("html-formatter-indent-width"), argument("NUMBER"), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The line ending applied to HTML and languages that extend it. If unset, inherits the global
    /// line ending.
    #[cfg_attr(
        feature = "cli",
        bpaf(
            long("html-formatter-line-ending"),
            argument("lf|crlf|cr|auto"),
            optional
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// The preferred maximum line width for HTML and languages that extend it. If unset, inherits
    /// the global line width.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("html-formatter-line-width"), argument("NUMBER"), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// The attribute position style in HTML elements. If unset, inherits the global attribute
    /// position setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(
            long("html-formatter-attribute-position"),
            argument("multiline|auto"),
            optional
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_position: Option<AttributePosition>,

    /// Controls the placement of the closing bracket for multiline HTML opening tags. Biome places
    /// the bracket at the end of the last attribute line when enabled and on its own line after the
    /// last attribute when disabled. This option also affects self-closing HTML elements. If unset,
    /// inherits the global `bracketSameLine` setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(
            long("html-formatter-bracket-same-line"),
            argument("true|false"),
            optional
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_same_line: Option<BracketSameLine>,

    /// Controls how the formatter treats whitespace around text and child elements in HTML, Vue,
    /// Svelte, and Astro markup. The `ignore` setting should be used only when whitespace cannot
    /// affect rendered output. Defaults to `css`.
    #[cfg_attr(
        feature = "cli",
        bpaf(
            long("html-formatter-whitespace-sensitivity"),
            argument("css|strict|ignore"),
            optional
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitespace_sensitivity: Option<WhitespaceSensitivity>,

    /// Controls whether the content of `<script>` and `<style>` tags is indented by one level in
    /// HTML, Vue, Svelte, and Astro files. Defaults to `false`.
    #[cfg_attr(
        feature = "cli",
        bpaf(
            long("html-formatter-indent-script-and-style"),
            argument("true|false"),
            optional
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_script_and_style: Option<IndentScriptAndStyle>,

    /// Controls whether void elements are self-closed. Defaults to `never`.
    #[cfg_attr(
        feature = "cli",
        bpaf(
            long("html-formatter-self-close-void-elements"),
            argument("always|never"),
            optional
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_close_void_elements: Option<SelfCloseVoidElements>,

    /// Whether to add a trailing newline at the end of the file. Unlike other language-specific
    /// trailing newline settings, this option defaults to `true` instead of inheriting the global
    /// setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(
            long("html-formatter-trailing-newline"),
            argument("true|false"),
            optional
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<TrailingNewline>,
}

/// Options that change how the HTML linter behaves.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct HtmlLinterConfiguration {
    /// Enables or disables the linter for HTML and languages that extend it.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("html-linter-enabled"), argument("true|false"), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<HtmlLinterEnabled>,
}

/// Options that change how HTML assist behaves.
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct HtmlAssistConfiguration {
    /// Enables or disables assist actions for HTML and languages that extend it.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("html-assist-enabled"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<HtmlAssistEnabled>,
}
