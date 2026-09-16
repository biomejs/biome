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

/// Options applied to HTML files.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct HtmlConfiguration {
    /// Enables full support for HTML, Vue, Svelte, and Astro files.
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
    /// Enables parsing double text expressions such as `{{ expression }}` inside `.html` files.
    pub interpolation: Option<HtmlParseInterpolation>,

    /// Enables parsing Vue syntax (`v-if`, `v-bind`, etc.) in `.html` files. Enabling this option
    /// also enables `interpolation` implicitly.
    ///
    /// Biome will already automatically enable Vue parsing in `.vue` files, so you probably don't need
    /// to enable this option. This only affects `.html` files, and does not change how `.vue`, `.svelte`,
    /// or `.astro` files are parsed.
    pub vue: Option<HtmlParseVue>,
}

/// Options that change how the HTML formatter behaves.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct HtmlFormatterConfiguration {
    /// Controls the formatter for HTML and languages that extend it.
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

    /// The maximum line width for HTML and languages that extend it. If unset, inherits the global
    /// line width.
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

    /// Whether to place the closing bracket of a multiline HTML tag at the end of the last line
    /// instead of on its own line. If unset, inherits the global `bracketSameLine` setting.
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

    /// Whether to account for whitespace sensitivity when formatting HTML and languages that
    /// extend it. Defaults to `css`.
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

    /// Whether to indent `<script>` and `<style>` tags in HTML and languages that extend it.
    /// Defaults to `false`.
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
    /// Controls the linter for HTML and languages that extend it.
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
    /// Controls assist actions for HTML and languages that extend it.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("html-assist-enabled"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<HtmlAssistEnabled>,
}
