use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::TsExternalModuleDeclarationFields;
use biome_js_syntax::{AnyTsExternalModuleDeclarationBody, TsExternalModuleDeclaration};

#[derive(Debug, Clone, Default)]
pub struct FormatTsExternalModuleDeclaration;

impl FormatNodeRule<TsExternalModuleDeclaration> for FormatTsExternalModuleDeclaration {
    fn fmt_fields(
        &self,
        node: &TsExternalModuleDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsExternalModuleDeclarationFields {
            body,
            module_token,
            source,
        } = node.as_fields();

        write!(f, [module_token.format(), space(), source.format(),])?;

        match body {
            Some(AnyTsExternalModuleDeclarationBody::TsEmptyExternalModuleDeclarationBody(
                body,
            )) => {
                body.format().fmt(f)?;
            }
            Some(AnyTsExternalModuleDeclarationBody::TsModuleBlock(body)) => {
                write!(f, [space(), body.format()])?;
            }
            None if f.options().semicolons().is_always() => {
                write!(f, [text(";")])?;
            }
            None => {}
        }

        Ok(())
    }
}
