use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_html_syntax::{SvelteName, SvelteNameFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteName {
    /// Whether it should be formatted in compact mode. In compact mode, all tokens and children
    /// are removed
    pub compact: bool,
}
impl FormatNodeRule<SvelteName> for FormatSvelteName {
    fn fmt_fields(&self, node: &SvelteName, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteNameFields { ident_token } = node.as_fields();
        if self.compact {
            let ident_token = ident_token?;
            format_removed(&ident_token).fmt(f)
        } else {
            write!(f, [ident_token.format()])
        }
    }
}

impl FormatRuleWithOptions<SvelteName> for FormatSvelteName {
    type Options = bool;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.compact = options;
        self
    }
}
