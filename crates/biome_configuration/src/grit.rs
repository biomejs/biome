use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth, TrailingNewline};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to GritQL files
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GritConfiguration {
    /// Formatting options
    #[bpaf(external(grit_formatter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<GritFormatterConfiguration>,

    /// Formatting options
    #[bpaf(external(grit_linter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<GritLinterConfiguration>,

    /// Assist options
    #[bpaf(external(grit_assist_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<GritAssistConfiguration>,
}

pub type GritFormatterEnabled = Bool<true>;

#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GritFormatterConfiguration {
    /// Control the formatter for Grit files.
    #[bpaf(long("grit-formatter-enabled"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<GritFormatterEnabled>,

    /// The indent style applied to Grit files.
    #[bpaf(long("grit-formatter-indent-style"), argument("tab|space"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to Grit files. Default to 2.
    #[bpaf(long("grit-formatter-indent-width"), argument("NUMBER"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to Grit files.
    #[bpaf(long("grit-formatter-line-ending"), argument("lf|crlf|cr"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to Grit files. Defaults to 80.
    #[bpaf(long("grit-formatter-line-width"), argument("NUMBER"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// Whether to add a trailing newline at the end of the file.
    ///
    /// Setting this option to `false` is **highly discouraged** because it could cause many problems with other tools:
    /// - <https://thoughtbot.com/blog/no-newline-at-end-of-file>
    /// - <https://callmeryan.medium.com/no-newline-at-end-of-file-navigating-gits-warning-for-android-developers-af14e73dd804>
    /// - <https://unix.stackexchange.com/questions/345548/how-to-cat-files-together-adding-missing-newlines-at-end-of-some-files>
    ///
    /// Disable the option at your own risk.
    ///
    /// Defaults to true.
    #[bpaf(
        long("grit-formatter-trailing-newline"),
        argument("true|false"),
        optional
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<TrailingNewline>,
}

pub type GritLinterEnabled = Bool<true>;

#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GritLinterConfiguration {
    /// Control the linter for Grit files.
    #[bpaf(long("grit-linter-enabled"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<GritLinterEnabled>,
}

pub type GritAssistEnabled = Bool<true>;
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GritAssistConfiguration {
    /// Control the assist functionality for Grit files.
    #[bpaf(long("grit-assist-enabled"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<GritAssistEnabled>,
}
