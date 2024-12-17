use crate::prelude::*;
use biome_css_syntax::CssSelectorList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSelectorList;
impl FormatRule<CssSelectorList> for FormatCssSelectorList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssSelectorList, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = soft_line_break_or_space();
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
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            // The selector case here like:
            // .a b {}
            // .a is a left computed selector
            let computed_selector =
                formatted
                    .node()?
                    .as_css_complex_selector()
                    .and_then(|complex_selector| {
                        complex_selector
                            .left()
                            .ok()?
                            .as_css_compound_selector()
                            .cloned()
                    });

            if let Some(computed_selector) = computed_selector {
                // Case like:
                // .a b {}
                // /* some long comment */
                // .a c {}
                //
                // if .a has leading comments, it should be formatted without indent.
                // Otherwise the formatted result will be like:
                // .a b {}
                // /* some long comment */
                //   .a c {}
                let simple_selector_has_leading_comments = computed_selector
                    .simple_selector()
                    .and_then(|simple_selector| simple_selector.as_css_type_selector().cloned())
                    .and_then(|type_selector| type_selector.ident().ok()?.value_token().ok())
                    .is_some_and(|value_token| value_token.has_leading_comments());

                // Sub selector same as the Simple Selector above:
                // .a b {}
                // /* some long comment */
                // d c {}
                // d is the Sub selector, so we also need to check if it has leading comments.
                let sub_selector_has_leading_comments = computed_selector
                    .sub_selectors()
                    .first()
                    .and_then(|sub_selector| sub_selector.as_css_class_selector().cloned())
                    .and_then(|class_selector| class_selector.dot_token().ok())
                    .is_some_and(|value_token| value_token.has_leading_comments());

                if simple_selector_has_leading_comments || sub_selector_has_leading_comments {
                    joiner.entry(&group(&formatted));
                    continue;
                }
            }

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
