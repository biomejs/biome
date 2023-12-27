use crate::prelude::*;
use crate::utils::{AnyJsAssignmentLike, FormatStatementSemicolon};
use biome_formatter::write;
use biome_js_syntax::TsTypeAliasDeclaration;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeAliasDeclaration;

impl FormatNodeRule<TsTypeAliasDeclaration> for FormatTsTypeAliasDeclaration {
    fn fmt_fields(&self, node: &TsTypeAliasDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let type_token = node.type_token()?;
        let semicolon = node.semicolon_token();
        let assignment_like = format_with(|f| write!(f, [AnyJsAssignmentLike::from(node.clone())]));
        write!(
            f,
            [
                type_token.format(),
                space(),
                group(&assignment_like),
                FormatStatementSemicolon::new(semicolon.as_ref())
            ]
        )
    }
}
