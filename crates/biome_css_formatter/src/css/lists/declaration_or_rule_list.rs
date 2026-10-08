use crate::prelude::*;
use biome_css_syntax::{AnyCssDeclarationOrRule, CssDeclarationOrRuleList};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationOrRuleList;
impl FormatRule<CssDeclarationOrRuleList> for FormatCssDeclarationOrRuleList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssDeclarationOrRuleList, f: &mut CssFormatter) -> FormatResult<()> {
        // This is one of the few cases where we _do_ want to respect empty
        // lines from the input, so we can use `join_nodes_with_hardline`.
        let mut join = f.join_nodes_with_hardline();

        let mut items = node.iter().peekable();
        while let Some(declaration_or_rule) = items.next() {
            // The semicolon after a metavariable stays on the same line:
            //
            // ```css
            // ${truncate};
            // ```
            let semicolon = if matches!(
                declaration_or_rule,
                AnyCssDeclarationOrRule::CssMetavariable(_)
            ) {
                items
                    .next_if(|next| matches!(next, AnyCssDeclarationOrRule::CssEmptyDeclaration(_)))
            } else {
                None
            };

            join.entry(
                declaration_or_rule.syntax(),
                &format_with(|f| {
                    write!(f, [format_or_verbatim(declaration_or_rule.format())])?;
                    if let Some(semicolon) = &semicolon {
                        write!(f, [format_or_verbatim(semicolon.format())])?;
                    }
                    Ok(())
                }),
            );
        }

        join.finish()
    }
}
