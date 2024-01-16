use crate::{prelude::*, utils::properties::FormatPropertyValueFields};
use biome_css_syntax::{CssBorder, CssBorderFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBorder;
impl FormatNodeRule<CssBorder> for FormatCssBorder {
    fn fmt_fields(&self, node: &CssBorder, f: &mut CssFormatter) -> FormatResult<()> {
        let CssBorderFields {
            line_width,
            line_style,
            color,
        } = node.as_fields();

        write!(
            f,
            [FormatPropertyValueFields::new(&format_args![
                line_width.format(),
                line_style.format(),
                color.format(),
            ])
            .with_slot_map(node.concrete_order_slot_map())]
        )
    }
}
