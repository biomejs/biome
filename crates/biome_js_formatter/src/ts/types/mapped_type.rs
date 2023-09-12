use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use crate::utils::FormatOptionalSemicolon;
use biome_formatter::trivia::FormatLeadingComments;
use biome_formatter::{format_args, write};
use biome_js_syntax::{JsSyntaxNode, TsMappedType, TsMappedTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsMappedType;

impl FormatNodeRule<TsMappedType> for FormatTsMappedType {
    fn fmt_fields(&self, node: &TsMappedType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsMappedTypeFields {
            l_curly_token,
            readonly_modifier,
            l_brack_token,
            property_name,
            in_token,
            keys_type,
            as_clause,
            r_brack_token,
            optional_modifier,
            mapped_type,
            semicolon_token,
            r_curly_token,
        } = node.as_fields();

        let property_name = property_name?;

        // Check if the user introduced a new line inside the node.
        let should_expand = node
            .syntax()
            .tokens()
            // Skip the first token to avoid formatter instability. See #4165.
            // This also makes sense since leading trivia of the first token
            // are not part of the interior of the node.
            .skip(1)
            .flat_map(|token| {
                token
                    .leading_trivia()
                    .pieces()
                    .chain(token.trailing_trivia().pieces())
            })
            .any(|piece| piece.is_newline());

        let comments = f.comments().clone();
        let dangling_comments = comments.dangling_comments(node.syntax());
        let type_annotation_has_leading_comment =
            mapped_type.as_ref().map_or(false, |annotation| {
                comments.has_leading_comments(annotation.syntax())
            });

        let format_inner = format_with(|f| {
            if let Some(readonly_modifier) = &readonly_modifier {
                write!(f, [readonly_modifier.format(), space()])?;
            }

            write!(
                f,
                [
                    FormatLeadingComments::Comments(dangling_comments),
                    group(&format_args![
                        l_brack_token.format(),
                        property_name.format(),
                        space(),
                        in_token.format(),
                        soft_line_indent_or_space(&format_args![
                            keys_type.format(),
                            as_clause.as_ref().map(|_| space()),
                            as_clause.format(),
                        ]),
                        r_brack_token.format(),
                    ]),
                    optional_modifier.format(),
                    type_annotation_has_leading_comment.then_some(space()),
                    mapped_type.format(),
                    if_group_breaks(&FormatOptionalSemicolon::new(semicolon_token.as_ref()))
                ]
            )
        });

        write!(
            f,
            [
                &l_curly_token.format(),
                group(&soft_space_or_block_indent(&format_inner)).should_expand(should_expand),
                r_curly_token.format(),
            ]
        )
    }

    fn needs_parentheses(&self, item: &TsMappedType) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(&self, _: &TsMappedType, _: &mut JsFormatter) -> FormatResult<()> {
        // Handled inside of `fmt_fields`
        Ok(())
    }
}

impl NeedsParentheses for TsMappedType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
