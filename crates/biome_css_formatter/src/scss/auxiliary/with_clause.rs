use crate::prelude::*;
use biome_css_syntax::{ScssForwardAtRule, ScssWithClause, ScssWithClauseFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssWithClause;

impl FormatNodeRule<ScssWithClause> for FormatScssWithClause {
    fn fmt_fields(&self, node: &ScssWithClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssWithClauseFields {
            with_token,
            configurations,
        } = node.as_fields();

        if is_trailing_forward_with_clause(node) {
            write!(f, [with_token.format(), configurations.format()])
        } else if is_standalone_forward_with_clause(node) {
            write!(f, [with_token.format(), hard_space(), configurations.format()])
        } else {
            write!(
                f,
                [with_token.format(), space(), configurations.format()]
            )
        }
    }
}

fn is_standalone_forward_with_clause(node: &ScssWithClause) -> bool {
    node.syntax()
        .parent()
        .and_then(|parent| ScssForwardAtRule::cast_ref(&parent))
        .is_some_and(|forward| {
            let fields = forward.as_fields();
            fields.as_clause.is_none() && fields.visibility_clause.is_none()
        })
}

fn is_trailing_forward_with_clause(node: &ScssWithClause) -> bool {
    node.syntax()
        .parent()
        .and_then(|parent| ScssForwardAtRule::cast_ref(&parent))
        .is_some_and(|forward| {
            let fields = forward.as_fields();
            fields.as_clause.is_some() || fields.visibility_clause.is_some()
        })
}
