use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_formatter::FormatRuleWithOptions;
use biome_formatter::separated::TrailingSeparator;
use biome_toml_syntax::TomlInlineTableMemberList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlInlineTableMemberList {
    layout: InlineTableLayout,
}

/// How the members of an inline table are laid out, chosen by the inline table.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(crate) enum InlineTableLayout {
    /// `{ a = 1, b = 2 }`, without a trailing comma, which TOML 1.0 doesn't
    /// allow.
    #[default]
    SingleLine,
    /// One member per line, each followed by a comma.
    Multiline,
}

impl FormatRuleWithOptions<TomlInlineTableMemberList> for FormatTomlInlineTableMemberList {
    type Options = InlineTableLayout;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.layout = options;
        self
    }
}

impl FormatRule<TomlInlineTableMemberList> for FormatTomlInlineTableMemberList {
    type Context = TomlFormatContext;

    fn fmt(&self, node: &TomlInlineTableMemberList, f: &mut TomlFormatter) -> FormatResult<()> {
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
