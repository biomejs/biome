use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth, TrailingNewline};
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to GritQL files.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GritConfiguration {
    /// GritQL formatter options.
    #[cfg_attr(
        feature = "cli",
        bpaf(external(grit_formatter_configuration), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<GritFormatterConfiguration>,

    /// GritQL linter options.
    #[cfg_attr(feature = "cli", bpaf(external(grit_linter_configuration), optional))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<GritLinterConfiguration>,

    /// GritQL assist options.
    #[cfg_attr(feature = "cli", bpaf(external(grit_assist_configuration), optional))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<GritAssistConfiguration>,
}

pub type GritFormatterEnabled = Bool<true>;

/// Options that change how the GritQL formatter behaves.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GritFormatterConfiguration {
    /// Controls the formatter for GritQL files.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("grit-formatter-enabled"), argument("true|false"), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<GritFormatterEnabled>,

    /// The indent style applied to GritQL files. If unset, inherits the global indentation style.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("grit-formatter-indent-style"), argument("tab|space"), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The indentation width applied to GritQL files. If unset, inherits the global indentation
    /// width.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("grit-formatter-indent-width"), argument("NUMBER"), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The line ending applied to GritQL files. If unset, inherits the global line ending.
    #[cfg_attr(
        feature = "cli",
        bpaf(
            long("grit-formatter-line-ending"),
            argument("lf|crlf|cr|auto"),
            optional
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// The maximum line width for GritQL files. If unset, inherits the global line width.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("grit-formatter-line-width"), argument("NUMBER"), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// Whether to add a trailing newline at the end of the file. If unset, inherits the global
    /// trailing newline setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(
            long("grit-formatter-trailing-newline"),
            argument("true|false"),
            optional
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<TrailingNewline>,
}

pub type GritLinterEnabled = Bool<true>;

/// Options that change how the GritQL linter behaves.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GritLinterConfiguration {
    /// Controls the linter for GritQL files.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("grit-linter-enabled"), argument("true|false"), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<GritLinterEnabled>,
}

pub type GritAssistEnabled = Bool<true>;

/// Options that change how GritQL assist behaves.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GritAssistConfiguration {
    /// Controls assist actions for GritQL files.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("grit-assist-enabled"), argument("true|false"), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<GritAssistEnabled>,
}
