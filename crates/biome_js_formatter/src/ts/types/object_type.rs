use crate::prelude::*;
use crate::utils::JsObjectLike;

use biome_formatter::write;
use biome_js_syntax::TsObjectType;

#[derive(Debug, Clone, Default)]
pub struct FormatTsObjectType;

impl FormatNodeRule<TsObjectType> for FormatTsObjectType {
    fn fmt_fields(&self, node: &TsObjectType, f: &mut JsFormatter) -> FormatResult<()> {
        write!(f, [JsObjectLike::from(node.clone())])
    }

    fn fmt_dangling_comments(&self, _: &TsObjectType, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted inside of `JsObjectLike`
        Ok(())
    }
}
