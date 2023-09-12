use crate::prelude::*;
use crate::utils::FormatInitializerClause;

use biome_formatter::write;
use biome_js_syntax::{TsEnumMember, TsEnumMemberFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsEnumMember;

impl FormatNodeRule<TsEnumMember> for FormatTsEnumMember {
    fn fmt_fields(&self, node: &TsEnumMember, f: &mut JsFormatter) -> FormatResult<()> {
        let TsEnumMemberFields { name, initializer } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                FormatInitializerClause::new(initializer.as_ref())
            ]
        )
    }
}
