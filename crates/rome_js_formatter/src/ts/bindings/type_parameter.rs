use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsTypeParameter, TsTypeParameterFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeParameter;

impl FormatNodeRule<TsTypeParameter> for FormatTsTypeParameter {
    fn fmt_fields(&self, node: &TsTypeParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeParameterFields {
            name,
            constraint,
            default,
            modifiers,
        } = node.as_fields();

        if !modifiers.is_empty() {
            write!(f, [modifiers.format(), space()])?;
        }

        write!(f, [name.format()])?;
        if let Some(constraint) = constraint {
            write!(f, [space(), constraint.format()])?;
        }

        if let Some(default) = default {
            write!(f, [space(), default.format()])?;
        }

        Ok(())
    }
}
