use crate::prelude::*;
use biome_css_syntax::{ScssAtRootAtRule, ScssAtRootAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssAtRootAtRule;

impl FormatNodeRule<ScssAtRootAtRule> for FormatScssAtRootAtRule {
    fn fmt_fields(&self, node: &ScssAtRootAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssAtRootAtRuleFields {
            at_root_token,
            query,
            selector,
            block,
        } = node.as_fields();

        write!(f, [at_root_token.format()])?;

        if let Some(query) = query {
            write!(f, [space(), query.format()])?;
        }

        if let Some(selector) = selector {
            write!(f, [space(), group(&selector.format())])?;
        }

        write!(f, [space(), block.format()])
    }
}
