use crate::prelude::*;
use crate::utils::comment_trivia::has_line_comment;
use biome_css_syntax::{
    AnyCssSelector, CssComplexSelector, CssComplexSelectorFields, CssSyntaxKind,
};
use biome_formatter::{FormatRuleWithOptions, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComplexSelector {
    options: ComplexSelectorOptions,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ComplexSelectorOptions {
    force_flat: bool,
}

impl FormatRuleWithOptions<CssComplexSelector> for FormatCssComplexSelector {
    type Options = ComplexSelectorOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatNodeRule<CssComplexSelector> for FormatCssComplexSelector {
    fn fmt_fields(&self, node: &CssComplexSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssComplexSelectorFields {
            left,
            combinator,
            right,
        } = node.as_fields();

        let left = left?;
        let combinator = combinator?;
        let right = right?;

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

        if let Some(computed_selector) = left.as_css_compound_selector() {
            let simple_selector_has_leading_comments = computed_selector
                .simple_selector()
                .and_then(|simple_selector| simple_selector.as_css_type_selector().cloned())
                .and_then(|type_selector| {
                    type_selector
                        .ident()
                        .ok()?
                        .as_css_identifier()
                        .and_then(|ident| ident.value_token().ok())
                })
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

        // The complex selector
        // /* comment longlonglonglong */
        // .a c {}
        // get formatted to
        // /* comment longlonglonglong */
        // .a
        //  c {}
        let force_flat = self.options.force_flat || has_trailing_separator_line_comment(node);
        let child_options = ComplexSelectorOptions { force_flat };
        let should_flatten = (has_leading_comments && !is_selector_list_first_child) || force_flat;
        let selector_separator = format_once(|f| {
            if should_flatten {
                write!(f, [space()])
            } else {
                write!(f, [soft_line_break_or_space()])
            }
        });

        write!(
            f,
            [
                format_selector(&left, child_options),
                selector_separator,
                formatted_combinator,
                space(),
                format_selector(&right, child_options)
            ]
        )
    }
}

fn format_selector(
    selector: &AnyCssSelector,
    options: ComplexSelectorOptions,
) -> impl Format<CssFormatContext> + '_ {
    format_with(move |f| match selector {
        AnyCssSelector::CssComplexSelector(selector) => {
            selector.format().with_options(options).fmt(f)
        }
        _ => selector.format().fmt(f),
    })
}

fn has_trailing_separator_line_comment(node: &CssComplexSelector) -> bool {
    let Some(comma_token) = node
        .syntax()
        .last_token()
        .and_then(|token| token.next_token())
        .filter(|token| token.kind() == CssSyntaxKind::COMMA)
    else {
        return false;
    };

    has_line_comment(comma_token.trailing_trivia())
}
