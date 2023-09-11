use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::TsImportTypeFields;
use biome_js_syntax::{JsSyntaxNode, TsImportType};

#[derive(Debug, Clone, Default)]
pub struct FormatTsImportType;

impl FormatNodeRule<TsImportType> for FormatTsImportType {
    fn fmt_fields(&self, node: &TsImportType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsImportTypeFields {
            typeof_token,
            import_token,
            l_paren_token,
            argument_token,
            r_paren_token,
            qualifier_clause,
            type_arguments,
        } = node.as_fields();

        if let Some(typeof_token) = typeof_token {
            write!(f, [typeof_token.format(), space()])?;
        }

        write![
            f,
            [
                import_token.format(),
                l_paren_token.format(),
                FormatLiteralStringToken::new(
                    &argument_token?,
                    StringLiteralParentKind::Expression
                ),
                r_paren_token.format(),
                qualifier_clause.format(),
                type_arguments.format(),
            ]
        ]
    }

    fn needs_parentheses(&self, item: &TsImportType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsImportType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
