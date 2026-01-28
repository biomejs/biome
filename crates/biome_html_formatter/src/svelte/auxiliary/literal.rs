use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_html_syntax::SvelteLiteral;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteLiteral {
    /// Whether it should be formatted in compact mode. In compact mode, all tokens and children
    /// are removed
    pub compact: bool,
}
impl FormatNodeRule<SvelteLiteral> for FormatSvelteLiteral {
    fn fmt_fields(&self, node: &SvelteLiteral, f: &mut HtmlFormatter) -> FormatResult<()> {
        if self.compact {
            let value_token = node.value_token()?;
            format_removed(&value_token).fmt(f)
        } else {
            write!(f, [node.value_token().format()])
        }
    }
}

impl FormatRuleWithOptions<SvelteLiteral> for FormatSvelteLiteral {
    type Options = bool;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.compact = options;
        self
    }
}
