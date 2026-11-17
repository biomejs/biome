use crate::prelude::*;
use biome_css_syntax::CssRelativeSelectorList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRelativeSelectorList;
impl FormatRule<CssRelativeSelectorList> for FormatCssRelativeSelectorList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssRelativeSelectorList, f: &mut CssFormatter) -> FormatResult<()> {
        // Using `join_with` instead of `join_nodes_with_soft_line` to avoid
        // preserving empty lines from the input source. See the comment in
        // [FormatCssSelectorList] for more information.
        let separator = soft_line_break_or_space();
        let comments = f.comments().clone();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            let entry = formatted.node()?.syntax();

            // Only a comment attached to the entry itself is printed inside it. A
            // comment before the first selector of a rule belongs to the rule, so
            // the selector keeps its indent when it wraps.
            let has_leading_comments = comments.has_leading_comments(entry)
                && entry
                    .first_token()
                    .is_some_and(|token| token.has_leading_comments());

            if has_leading_comments {
                // The entry prints its own leading comments, which already set the
                // indentation of the selector that follows them. Indenting the entry
                // as well would move the selector away from its comment. The kind of
                // selector that comes first (`div`, `.a`, `&`, a combinator) doesn't
                // matter.
                joiner.entry(&group(&formatted));
            } else {
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
        }

        joiner.finish()
    }
}
