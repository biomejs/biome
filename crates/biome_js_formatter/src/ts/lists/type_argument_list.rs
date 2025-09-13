use crate::prelude::*;
use crate::separated::JsFormatSeparatedElementRule;
use biome_formatter::separated::FormatSeparatedIter;
use biome_js_syntax::TsTypeArgumentList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeArgumentList;

impl FormatRule<TsTypeArgumentList> for FormatTsTypeArgumentList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsTypeArgumentList, f: &mut JsFormatter) -> FormatResult<()> {
        // When a trailing comma is present, the actual last element is not the comma but a missing element.
        // To omit the trailing comma, drop the missing element.
        let mut elements: Vec<_> = node.elements().collect();
        if matches!(elements.last().map(|e| e.node()), Some(Err(_))) {
            elements.pop();
        }
        let entries = FormatSeparatedIter::new(
            elements.into_iter(),
            ",",
            JsFormatSeparatedElementRule::new(),
            on_skipped,
            on_removed,
        )
        .with_trailing_separator(TrailingSeparator::Omit);
        f.join_with(&soft_line_break_or_space())
            .entries(entries)
            .finish()
    }
}
