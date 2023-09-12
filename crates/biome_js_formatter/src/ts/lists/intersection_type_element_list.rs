use crate::{prelude::*, utils::is_object_like_type};
use biome_formatter::{format_args, write};
use biome_js_syntax::TsIntersectionTypeElementList;
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsIntersectionTypeElementList;

impl FormatRule<TsIntersectionTypeElementList> for FormatTsIntersectionTypeElementList {
    type Context = JsFormatContext;

    // [Prettier applies]: https://github.com/prettier/prettier/blob/cd3e530c2e51fb8296c0fb7738a9afdd3a3a4410/src/language-js/print/type-annotation.js#L93-L120
    fn fmt(&self, node: &TsIntersectionTypeElementList, f: &mut JsFormatter) -> FormatResult<()> {
        let last_index = node.len().saturating_sub(1);

        let mut is_prev_object_type_like = false;
        let mut is_chain_indented = false;

        for (index, element) in node.elements().enumerate() {
            let node = element.node()?;

            let is_object_type_like = is_object_like_type(node);

            // always inline first element
            if index == 0 {
                write!(f, [node.format()])?;
            } else {
                // If no object is involved, go to the next line if it breaks
                if !is_prev_object_type_like && !is_object_type_like {
                    write!(
                        f,
                        [indent(&format_args![
                            soft_line_break_or_space(),
                            node.format()
                        ])]
                    )?;
                } else {
                    write!(f, [space()])?;

                    if !is_prev_object_type_like || !is_object_type_like {
                        // indent if we move from object to non-object or vice versa, otherwise keep inline
                        is_chain_indented = index > 1;
                    }

                    if is_chain_indented {
                        write!(f, [indent(&node.format())])?;
                    } else {
                        write!(f, [node.format()])?;
                    }
                }
            }

            let trailing_separator = element.trailing_separator()?;

            if let Some(token) = trailing_separator {
                if index == last_index {
                    write![f, [format_removed(token)]]?;
                } else {
                    write![f, [space(), token.format()]]?;
                }
            }

            is_prev_object_type_like = is_object_type_like;
        }

        Ok(())
    }
}
