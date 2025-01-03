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
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            let has_leading_comments = formatted
                .node()?
                .as_css_relative_selector()
                .and_then(|relative_selector| {
                    relative_selector
                        .selector()
                        .ok()?
                        .as_css_compound_selector()
                        .cloned()
                })
                .and_then(|computed_selector| computed_selector.simple_selector())
                .and_then(|simple_selector| simple_selector.as_css_type_selector().cloned())
                .and_then(|type_selector| type_selector.ident().ok()?.value_token().ok())
                .is_some_and(|value_token| value_token.has_leading_comments());

            if has_leading_comments {
                // Computed Selector which contains a leading comments should be formatted without indent.
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
