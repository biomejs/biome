use crate::prelude::*;
use biome_css_syntax::CssDeclarationList;
use biome_formatter::separated::TrailingSeparator;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationList;
impl FormatRule<CssDeclarationList> for FormatCssDeclarationList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssDeclarationList, f: &mut CssFormatter) -> FormatResult<()> {
        // This is one of the few cases where we _do_ want to respect empty
        // lines from the input, so we can use `join_nodes_with_hardline`.
        let mut joiner = f.join_nodes_with_hardline();

        for (rule, formatted) in node.elements().zip(
            node.format_separated(";")
                .with_trailing_separator(TrailingSeparator::Mandatory),
        ) {
            joiner.entry(rule.node()?.syntax(), &formatted);
        }

        joiner.finish()
    }
}
