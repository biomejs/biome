use crate::prelude::*;
use biome_css_syntax::CssKeyframesSelectorList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesSelectorList;
impl FormatRule<CssKeyframesSelectorList> for FormatCssKeyframesSelectorList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssKeyframesSelectorList, f: &mut CssFormatter) -> FormatResult<()> {
        // Using `join_with` and a manual separator instead of `join_nodes_with_soft_line`
        // here allows ensures that the list won't try to read the input source to
        // preserve line breaks and empty lines. That way, an input like:
        //     h1,
        //
        //     h2 {}
        // gets formatted to
        //     h1, h2 {}
        // and only breaks when the result doesn't fit on a single line.
        //
        // For top-level selector lists, like as the `prelude` of a `CssRule`, the
        // parent node can wrap this list in a group and set `should_expand(true)`
        // to have the separator expand into full line breaks instead of trying to
        // fit on a single line.
        let separator = soft_line_break_or_space();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
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
            joiner.entry(&group(&indent(&formatted)));
        }

        joiner.finish()
    }
}
