use biome_deserialize::StringSet;
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use biome_formatter::{
    AttributePosition, BracketSpacing, IndentStyle, IndentWidth, LineEnding, LineWidth,
};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Generic options applied to all files
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct FormatterConfiguration {
    // if `false`, it disables the feature. `true` by default
    #[partial(bpaf(hide))]
    pub enabled: bool,

    #[partial(bpaf(long("use-editorconfig"), argument("true|false"), optional))]
    /// Use any `.editorconfig` files to configure the formatter. Configuration in `biome.json` will override `.editorconfig` configuration. Default: false.
    pub use_editorconfig: bool,

    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    #[partial(bpaf(hide))]
    pub format_with_errors: bool,

    /// The indent style.
    #[partial(bpaf(long("indent-style"), argument("tab|space"), optional))]
    pub indent_style: IndentStyle,

    /// The size of the indentation, 2 by default (deprecated, use `indent-width`)
    #[partial(bpaf(long("indent-size"), argument("NUMBER"), optional))]
    #[partial(deserializable(deprecated(use_instead = "formatter.indentWidth")))]
    pub indent_size: IndentWidth,

    /// The size of the indentation, 2 by default
    #[partial(bpaf(long("indent-width"), argument("NUMBER"), optional))]
    pub indent_width: IndentWidth,

    /// The type of line ending.
    #[partial(bpaf(long("line-ending"), argument("lf|crlf|cr"), optional))]
    pub line_ending: LineEnding,

    /// What's the max width of a line. Defaults to 80.
    #[partial(bpaf(long("line-width"), argument("NUMBER"), optional))]
    pub line_width: LineWidth,

    /// The attribute position style in HTMLish languages. By default auto.
    #[partial(bpaf(long("attribute-position"), argument("multiline|auto"), optional))]
    pub attribute_position: AttributePosition,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[partial(bpaf(long("bracket-spacing"), argument("true|false"), optional))]
    pub bracket_spacing: BracketSpacing,

    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[partial(bpaf(hide))]
    pub ignore: StringSet,

    /// A list of Unix shell style patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[partial(bpaf(hide))]
    pub include: StringSet,
}

impl PartialFormatterConfiguration {
    pub const fn is_disabled(&self) -> bool {
        matches!(self.enabled, Some(false))
    }

    pub fn get_formatter_configuration(&self) -> FormatterConfiguration {
        FormatterConfiguration {
            enabled: self.enabled.unwrap_or_default(),
            format_with_errors: self.format_with_errors.unwrap_or_default(),
            indent_style: self.indent_style.unwrap_or_default(),
            indent_size: self.indent_size.unwrap_or_default(),
            indent_width: self.indent_width.unwrap_or_default(),
            line_ending: self.line_ending.unwrap_or_default(),
            line_width: self.line_width.unwrap_or_default(),
            attribute_position: self.attribute_position.unwrap_or_default(),
            bracket_spacing: self.bracket_spacing.unwrap_or_default(),
            ignore: self.ignore.clone().unwrap_or_default(),
            include: self.include.clone().unwrap_or_default(),
            use_editorconfig: self.use_editorconfig.unwrap_or_default(),
        }
    }
}

impl Default for FormatterConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            format_with_errors: false,
            indent_size: IndentWidth::default(),
            indent_width: IndentWidth::default(),
            indent_style: IndentStyle::default(),
            line_ending: LineEnding::default(),
            line_width: LineWidth::default(),
            attribute_position: AttributePosition::default(),
            bracket_spacing: Default::default(),
            ignore: Default::default(),
            include: Default::default(),
            // TODO: Biome 2.0: change to true
            use_editorconfig: Default::default(),
        }
    }
}

/// Required by [Bpaf].
impl FromStr for FormatterConfiguration {
    type Err = &'static str;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}
