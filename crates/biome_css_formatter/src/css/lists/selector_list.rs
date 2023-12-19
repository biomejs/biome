use crate::{prelude::*, separated::FormatAstSeparatedListExtension};
use biome_css_syntax::CssSelectorList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSelectorList;
impl FormatRule<CssSelectorList> for FormatCssSelectorList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssSelectorList, f: &mut CssFormatter) -> FormatResult<()> {
        let mut joiner = f.join_nodes_with_hardline();

        for (rule, formatted) in node.elements().zip(node.format_separated(",")) {
            // Each selector gets `indent` added in case it breaks over multiple
            // lines. The break is added here rather than in each selector both
            // for simplicity and to avoid recursively adding indents when
            // selectors are nested within other rules.
            //
            // For example, a selector like `div span a` is structured like
            // `[div, [span, [a]]]`, so `a` would end up double-indented if it
            // was handled by the selector rather than here.
            joiner.entry(rule.node()?.syntax(), &indent(&formatted));
        }

        joiner.finish()
    }
}
