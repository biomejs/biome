use crate::prelude::*;
use biome_css_syntax::{ScssKeyframesName, ScssKeyframesNameFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssKeyframesName;

impl FormatNodeRule<ScssKeyframesName> for FormatScssKeyframesName {
    fn fmt_fields(&self, node: &ScssKeyframesName, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssKeyframesNameFields { name } = node.as_fields();

        write!(f, [name.format()])
    }
}
