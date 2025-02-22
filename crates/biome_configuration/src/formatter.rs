use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, IndentStyle, IndentWidth, LineEnding,
    LineWidth, ObjectWrap,
};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

pub type FormatterEnabled = Bool<true>;
pub type UseEditorconfigEnabled = Bool<false>;
pub type FormatWithErrorsEnabled = Bool<false>;

/// Generic options applied to all files
#[derive(
    Bpaf, Clone, Deserializable, Debug, Default, Deserialize, Eq, PartialEq, Merge, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct FormatterConfiguration {
    // if `false`, it disables the feature. `true` by default
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<FormatterEnabled>,

    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_with_errors: Option<FormatWithErrorsEnabled>,

    /// The indent style.
    #[bpaf(long("indent-style"), argument("tab|space"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation, 2 by default
    #[bpaf(long("indent-width"), argument("NUMBER"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending.
    #[bpaf(long("line-ending"), argument("lf|crlf|cr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line. Defaults to 80.
    #[bpaf(long("line-width"), argument("NUMBER"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// The attribute position style in HTML-ish languages. Defaults to auto.
    #[bpaf(long("attribute-position"), argument("multiline|auto"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_position: Option<AttributePosition>,

    /// Put the `>` of a multi-line HTML or JSX element at the end of the last line instead of being alone on the next line (does not apply to self closing elements).
    #[bpaf(long("bracket-same-line"), argument("true|false"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_same_line: Option<BracketSameLine>,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[bpaf(long("bracket-spacing"), argument("true|false"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_spacing: Option<BracketSpacing>,

    /// Whether to enforce collapsing object literals when possible. Defaults to preserve.
    #[bpaf(long("object-wrap"), argument("preserve|collapse"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_wrap: Option<ObjectWrap>,

    /// Use any `.editorconfig` files to configure the formatter. Configuration
    /// in `biome.json` will override `.editorconfig` configuration.
    ///
    /// Default: `false`.
    #[bpaf(long("use-editorconfig"), argument("true|false"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_editorconfig: Option<UseEditorconfigEnabled>,

    /// A list of glob patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[bpaf(pure(Default::default()), hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<biome_glob::Glob>>,
}

impl FormatterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    pub fn format_with_errors_resolved(&self) -> bool {
        self.format_with_errors.unwrap_or_default().into()
    }

    pub fn indent_style_resolved(&self) -> IndentStyle {
        self.indent_style.unwrap_or_default()
    }

    pub fn indent_width_resolved(&self) -> IndentWidth {
        self.indent_width.unwrap_or_default()
    }

    pub fn line_ending_resolved(&self) -> LineEnding {
        self.line_ending.unwrap_or_default()
    }

    pub fn line_width_resolved(&self) -> LineWidth {
        self.line_width.unwrap_or_default()
    }

    pub fn attribute_position_resolved(&self) -> AttributePosition {
        self.attribute_position.unwrap_or_default()
    }

    pub fn bracket_spacing_resolved(&self) -> BracketSpacing {
        self.bracket_spacing.unwrap_or_default()
    }

    pub fn object_wrap_resolved(&self) -> ObjectWrap {
        self.object_wrap.unwrap_or_default()
    }

    pub fn use_editorconfig_resolved(&self) -> bool {
        self.use_editorconfig.unwrap_or_default().into()
    }
}
