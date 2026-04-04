use crate::prelude::*;
use biome_formatter::FormatRuleWithOptions;
use biome_html_syntax::SvelteDirectiveModifierList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteDirectiveModifierList {
    /// Whether it should be formatted in compact mode. In compact mode, all tokens and children
    /// are removed
    pub compact: bool,
}
impl FormatRule<SvelteDirectiveModifierList> for FormatSvelteDirectiveModifierList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &SvelteDirectiveModifierList, f: &mut HtmlFormatter) -> FormatResult<()> {
        let mut joiner = f.join();
        for entry in node.iter() {
            joiner.entry(&entry.format().with_options(self.compact));
        }

        joiner.finish()
    }
}

impl FormatRuleWithOptions<SvelteDirectiveModifierList> for FormatSvelteDirectiveModifierList {
    type Options = bool;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.compact = options;
        self
    }
}
