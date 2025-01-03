use biome_deserialize_macros::{Deserializable, Merge, Partial};
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to GritQL files
#[derive(Clone, Debug, Default, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct GritConfiguration {
    /// Formatting options
    #[partial(type, bpaf(external(partial_grit_formatter), optional))]
    pub formatter: GritFormatter,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct GritFormatter {
    /// Control the formatter for Grit files.
    #[partial(bpaf(long("grit-formatter-enabled"), argument("true|false"), optional))]
    pub enabled: bool,

    /// The indent style applied to Grit files.
    #[partial(bpaf(long("grit-formatter-indent-style"), argument("tab|space"), optional))]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to Grit files. Default to 2.
    #[partial(bpaf(long("grit-formatter-indent-width"), argument("NUMBER"), optional))]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to Grit files.
    #[partial(bpaf(long("grit-formatter-line-ending"), argument("lf|crlf|cr"), optional))]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to Grit files. Defaults to 80.
    #[partial(bpaf(long("grit-formatter-line-width"), argument("NUMBER"), optional))]
    pub line_width: Option<LineWidth>,
}
