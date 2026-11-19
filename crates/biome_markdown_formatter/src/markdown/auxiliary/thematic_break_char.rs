use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::MdThematicBreakChar;
use biome_markdown_syntax::thematic_break_ext::MdThematicBreakMarker;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdThematicBreakChar {
    replacement: Option<MdThematicBreakMarker>,
    should_remove: bool,
}
impl FormatNodeRule<MdThematicBreakChar> for FormatMdThematicBreakChar {
    fn fmt_fields(
        &self,
        node: &MdThematicBreakChar,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let token = node.value()?;

        if self.should_remove {
            return write!(f, [format_removed(&token)]);
        }

        if let Some(replacement) = self.replacement {
            return write!(
                f,
                [format_replaced(
                    &token,
                    &text(replacement.as_str(), Some(token.text_range().start()))
                )]
            );
        }

        token.format().fmt(f)
    }
}

pub(crate) struct FormatMdThematicBreakCharOptions {
    pub(crate) replacement: Option<MdThematicBreakMarker>,
    pub(crate) should_remove: bool,
}

impl FormatRuleWithOptions<MdThematicBreakChar> for FormatMdThematicBreakChar {
    type Options = FormatMdThematicBreakCharOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.replacement = options.replacement;
        self.should_remove = options.should_remove;
        self
    }
}
