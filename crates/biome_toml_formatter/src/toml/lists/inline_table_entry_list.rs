use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_formatter::FormatRuleWithOptions;
use biome_formatter::separated::TrailingSeparator;
use biome_toml_syntax::TomlInlineTableEntryList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlInlineTableEntryList {
    layout: InlineTableLayout,
}

/// How the entries of an inline table are laid out, chosen by the inline table.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(crate) enum InlineTableLayout {
    /// `{ a = 1, b = 2 }`, without a trailing comma, which TOML 1.0 doesn't
    /// allow.
    #[default]
    SingleLine,
    /// One entry per line, each followed by a comma.
    Multiline,
}

impl FormatRuleWithOptions<TomlInlineTableEntryList> for FormatTomlInlineTableEntryList {
    type Options = InlineTableLayout;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.layout = options;
        self
    }
}

impl FormatRule<TomlInlineTableEntryList> for FormatTomlInlineTableEntryList {
    type Context = TomlFormatContext;

    fn fmt(&self, node: &TomlInlineTableEntryList, f: &mut TomlFormatter) -> FormatResult<()> {
        match self.layout {
            InlineTableLayout::SingleLine => {
                let mut join = f.join_with(space());
                join.entries(node.format_separated(",", TrailingSeparator::Omit));
                join.finish()
            }
            InlineTableLayout::Multiline => {
                let mut join = f.join_nodes_with_hardline();
                for (element, formatted) in node
                    .elements()
                    .zip(node.format_separated(",", TrailingSeparator::Mandatory))
                {
                    join.entry(element.node()?.syntax(), &formatted);
                }
                join.finish()
            }
        }
    }
}
