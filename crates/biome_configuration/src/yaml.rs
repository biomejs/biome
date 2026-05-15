use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{IndentWidth, LineEnding, LineWidth, TrailingNewline};
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to Yaml files
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct YamlConfiguration {
    #[cfg_attr(
        feature = "cli",
        bpaf(external(yaml_formatter_configuration), optional, hide)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<YamlFormatterConfiguration>,
}

pub type YamlFormatterEnabled = Bool<false>; // Keep it disabled by default while experimental.
pub type YamlLinterEnabled = Bool<true>;
pub type YamlAssistEnabled = Bool<true>;
pub type YamlParseInterpolation = Bool<false>;

/// Options that change how the Yaml formatter behaves
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct YamlFormatterConfiguration {
    /// Control the formatter for Yaml (and its super languages) files.
    #[cfg_attr(all(feature = "cli", feature = "yaml"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<YamlFormatterEnabled>,

    /// The size of the indentation applied to Yaml files. Defaults to 2.
    #[cfg_attr(all(feature = "cli", feature = "yaml"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// What's the max width of a line applied to Yaml files. Defaults to 80.
    #[cfg_attr(all(feature = "cli", feature = "yaml"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// Whether to add a trailing newline at the end of the file.
    ///
    /// Setting this option to `false` is **highly discouraged** because it could cause many problems with other tools:
    /// - https://thoughtbot.com/blog/no-newline-at-end-of-file
    /// - https://callmeryan.medium.com/no-newline-at-end-of-file-navigating-gits-warning-for-android-developers-af14e73dd804
    /// - https://unix.stackexchange.com/questions/345548/how-to-cat-files-together-adding-missing-newlines-at-end-of-some-files
    ///
    /// Disable the option at your own risk.
    ///
    /// Defaults to true.
    #[cfg_attr(all(feature = "cli", feature = "yaml"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<TrailingNewline>,

    /// The type of line ending applied to Yaml (and its super languages) files. `auto` uses CRLF on Windows and LF on other platforms.
    #[cfg_attr(all(feature = "cli", feature = "yaml"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,
}
