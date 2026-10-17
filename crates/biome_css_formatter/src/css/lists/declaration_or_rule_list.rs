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
            // The semicolon after a metavariable, and the metavariables after
            // it on the same line, stay on the same line:
            //
            // ```css
            // ${truncate};
            // ${base} ${hover}
            // ```
            let mut same_line = Vec::new();
            if is_metavariable(&declaration_or_rule) {
                loop {
                    if let Some(semicolon) = items.next_if(|next| {
                        matches!(next, AnyCssDeclarationOrRule::CssEmptyDeclaration(_))
                    }) {
                        same_line.push(semicolon);
                    }

                    match items.next_if(|next| {
                        is_metavariable(next) && !next.syntax().has_leading_newline()
                    }) {
                        Some(metavariable) => same_line.push(metavariable),
                        None => break,
                    }
                }
            }

            join.entry(
                declaration_or_rule.syntax(),
                &format_with(|f| {
                    write!(f, [format_or_verbatim(declaration_or_rule.format())])?;
                    for item in &same_line {
                        if is_metavariable(item) {
                            write!(f, [space()])?;
                        }
                        write!(f, [format_or_verbatim(item.format())])?;
                    }
                    Ok(())
                }),
            );
        }

        join.finish()
    }
}

fn is_metavariable(declaration_or_rule: &AnyCssDeclarationOrRule) -> bool {
    matches!(
        declaration_or_rule,
        AnyCssDeclarationOrRule::CssMetavariable(_)
    )
}
