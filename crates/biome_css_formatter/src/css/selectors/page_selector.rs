use crate::prelude::*;
use biome_css_syntax::{CssPageSelector, CssPageSelectorFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPageSelector;
impl FormatNodeRule<CssPageSelector> for FormatCssPageSelector {
    fn fmt_fields(&self, node: &CssPageSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssPageSelectorFields { ty, pseudos } = node.as_fields();
        let has_leading_pseudo_comment = ty.is_some()
            && pseudos
                .iter()
                .next()
                .is_some_and(|pseudo| f.comments().has_leading_comments(pseudo.syntax()));

        write!(
            f,
            [group(&format_args![
                ty.format(),
                has_leading_pseudo_comment.then_some(space()),
                pseudos.format()
            ])]
        )
    }
}
