use crate::prelude::*;
use crate::utils::custom_property::has_source_gap;
use biome_css_syntax::CssCustomPropertyComponentList;
use biome_formatter::write;
use biome_rowan::{AstNode, AstNodeList};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyComponentList;
impl FormatRule<CssCustomPropertyComponentList> for FormatCssCustomPropertyComponentList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssCustomPropertyComponentList, f: &mut CssFormatter) -> FormatResult<()> {
        if node.is_empty() {
            // Empty lists own comments between their container delimiters.
            return format_dangling_comments(node.syntax()).fmt(f);
        }

        let mut previous = None;

        for component in node.iter() {
            let first = component.syntax().first_token();
            if let (Some(previous), Some(first)) = (previous.as_ref(), first.as_ref()) {
                if first.has_leading_newline() {
                    write!(f, [hard_line_break()])?;
                } else if has_source_gap(previous, first) {
                    write!(f, [space()])?;
                }
            }

            write!(f, [component.format()])?;
            previous = component.syntax().last_token();
        }

        Ok(())
    }
}
