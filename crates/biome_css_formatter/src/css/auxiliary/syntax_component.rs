use crate::prelude::*;
use biome_css_syntax::{CssSyntaxComponent, CssSyntaxComponentFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSyntaxComponent;

impl FormatNodeRule<CssSyntaxComponent> for FormatCssSyntaxComponent {
    fn fmt_fields(&self, node: &CssSyntaxComponent, f: &mut CssFormatter) -> FormatResult<()> {
        let CssSyntaxComponentFields {
            component,
            multiplier,
        } = node.as_fields();

        write!(f, [component.format(), multiplier.format()])
    }
}
