use crate::prelude::*;
use biome_css_syntax::{
    ScssForwardAtRule, ScssModuleConfigurationList, ScssModuleConfigurationListFields,
    ScssWithClause,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssModuleConfigurationList;

impl FormatNodeRule<ScssModuleConfigurationList> for FormatScssModuleConfigurationList {
    fn fmt_fields(
        &self,
        node: &ScssModuleConfigurationList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssModuleConfigurationListFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&items.format()),
                r_paren_token.format()
            ])
            .should_expand(should_expand_in_forward_with_clause(node))]
        )
    }
}

fn should_expand_in_forward_with_clause(node: &ScssModuleConfigurationList) -> bool {
    if node.items().len() <= 1 {
        return false;
    }

    node.syntax()
        .parent()
        .and_then(|parent| ScssWithClause::cast_ref(&parent))
        .and_then(|with_clause| with_clause.syntax().parent())
        .and_then(|parent| ScssForwardAtRule::cast_ref(&parent))
        .is_some_and(|forward| {
            let fields = forward.as_fields();
            fields.as_clause.is_none() && fields.visibility_clause.is_none()
        })
}
