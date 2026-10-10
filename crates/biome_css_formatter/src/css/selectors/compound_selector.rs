use crate::prelude::*;
use crate::verbatim::format_verbatim_skipped_body;
use biome_css_syntax::{AnyCssSubSelector, CssCompoundSelector, CssCompoundSelectorFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCompoundSelector;
impl FormatNodeRule<CssCompoundSelector> for FormatCssCompoundSelector {
    fn fmt_fields(&self, node: &CssCompoundSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssCompoundSelectorFields {
            nesting_selectors,
            simple_selector,
            sub_selectors,
        } = node.as_fields();

        let has_placeholder = sub_selectors
            .iter()
            .any(|item| matches!(item, AnyCssSubSelector::ScssPlaceholderSelector(_)));
        let has_boundary_comment = has_placeholder
            && nesting_selectors
                .iter()
                .map(|selector| selector.into_syntax())
                .chain(
                    simple_selector
                        .as_ref()
                        .map(|selector| selector.syntax().clone()),
                )
                .chain(sub_selectors.iter().map(|selector| selector.into_syntax()))
                .any(|selector| {
                    selector.first_token().is_some_and(|token| {
                        token.prev_token().is_some_and(|previous| {
                            node.syntax()
                                .text_range_with_trivia()
                                .contains_range(previous.text_range())
                                && (previous.has_trailing_comments()
                                    || token.has_leading_comments())
                        })
                    })
                });

        if has_boundary_comment {
            // Adding spaces to `%a/**/%b` creates a descendant in the CSS tree,
            // which changes compound-only pseudo arguments and chain wrapping.
            return format_verbatim_skipped_body(node.syntax()).fmt(f);
        }

        write!(
            f,
            [group(&format_args![
                nesting_selectors.format(),
                simple_selector.format(),
                sub_selectors.format()
            ])]
        )
    }
}
