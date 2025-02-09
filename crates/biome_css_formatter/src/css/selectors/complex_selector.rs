use crate::prelude::*;
use biome_css_syntax::{CssComplexSelector, CssComplexSelectorFields, CssSyntaxKind};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComplexSelector;
impl FormatNodeRule<CssComplexSelector> for FormatCssComplexSelector {
    fn fmt_fields(&self, node: &CssComplexSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssComplexSelectorFields {
            left,
            combinator,
            right,
        } = node.as_fields();

        let combinator = combinator?;

        // Space combinators only care that there is _some_ whitespace between
        // the two selectors. Here, it gets replaced by a soft_line_break_or_space
        // to allow the complete selector to break onto multiple lines if needed.
        let formatted_combinator = format_with(|f| {
            if matches!(combinator.kind(), CssSyntaxKind::CSS_SPACE_LITERAL) {
                write!(f, [format_removed(&combinator)])
            } else {
                write!(f, [combinator.format()])
            }
        });
        let mut has_leading_comments = false;

        if let Some(computed_selector) = left.clone()?.as_css_compound_selector() {
            let simple_selector_has_leading_comments = computed_selector
                .simple_selector()
                .and_then(|simple_selector| simple_selector.as_css_type_selector().cloned())
                .and_then(|type_selector| type_selector.ident().ok()?.value_token().ok())
                .is_some_and(|value_token| value_token.has_leading_comments());

            let sub_selector_has_leading_comments = computed_selector
                .sub_selectors()
                .first()
                .and_then(|sub_selector| sub_selector.as_css_class_selector().cloned())
                .and_then(|class_selector| class_selector.dot_token().ok())
                .is_some_and(|value_token| value_token.has_leading_comments());

            has_leading_comments =
                simple_selector_has_leading_comments || sub_selector_has_leading_comments;
        }

        // The selector list first child like the case:
        // .a b, .a c {}
        // Then the complex selector `.a b` should be the first child of the selector list.
        let is_selector_list_first_child = node.syntax().parent().is_some_and(|parent| {
            matches!(parent.kind(), CssSyntaxKind::CSS_SELECTOR_LIST)
                && node.syntax().prev_sibling().is_none()
        });

        // If the complex selector has leading comments and it's not the first child of the selector list,
        // don't insert `soft_line_break_or_space` before it, because the comments will cause the selector to break.
        // Otherwise the case like:
        // .a b,
        // /* comment longlonglonglong */
        // .a c {}

        // The complext selector
        // /* comment longlonglonglong */
        // .a c {}
        // get formatted to
        // /* comment longlonglonglong */
        // .a
        //  c {}
        if has_leading_comments && !is_selector_list_first_child {
            write!(
                f,
                [
                    left.format(),
                    space(),
                    formatted_combinator,
                    space(),
                    right.format()
                ]
            )
        } else {
            write!(
                f,
                [
                    left.format(),
                    soft_line_break_or_space(),
                    formatted_combinator,
                    space(),
                    right.format()
                ]
            )
        }
    }
}
