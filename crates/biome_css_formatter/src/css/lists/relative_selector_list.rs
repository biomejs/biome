use crate::prelude::*;
use biome_css_syntax::CssRelativeSelectorList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRelativeSelectorList;
impl FormatRule<CssRelativeSelectorList> for FormatCssRelativeSelectorList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssRelativeSelectorList, f: &mut CssFormatter) -> FormatResult<()> {
        let mut joiner = f.join_nodes_with_soft_line();

        for (rule, formatted) in node.elements().zip(node.format_separated(",")) {
            // Each selector gets `indent` added in case it breaks over multiple
            // lines. The break is added here rather than in each selector both
            // for simplicity and to avoid recursively adding indents when
            // selectors are nested within other rules. The group is then added
            // around the indent to ensure that it tries using a flat layout
            // first and only expands when the single selector can't fit the line.
            //
            // For example, a selector like `div span a` is structured like
            // `[div, [span, [a]]]`, so `a` would end up double-indented if it
            // was handled by the selector rather than here.
            joiner.entry(rule.node()?.syntax(), &group(&indent(&formatted)));
        }

        joiner.finish()
    }
}
