use crate::prelude::*;
use crate::utils::FormatOptionalSemicolon;

use biome_formatter::trivia::FormatLeadingComments;
use biome_formatter::{format_args, write};
use biome_js_syntax::{TsMappedType, TsMappedTypeFields};
use biome_rowan::Direction;

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
        let should_expand = has_line_break_before_property_name(node)?;

        let comments = f.comments().clone();
        let type_annotation_has_leading_comment = mapped_type
            .as_ref()
            .is_some_and(|annotation| comments.has_leading_comments(annotation.syntax()));

        let format_inner = format_with(|f| {
            write!(
                f,
                [FormatLeadingComments::Comments(
                    comments.dangling_comments(node.syntax())
                )]
            )?;

            if let Some(readonly_modifier) = &readonly_modifier {
                write!(f, [readonly_modifier.format(), space()])?;
            }

            write!(
                f,
                [
                    group(&format_args![
                        l_brack_token.format(),
                        property_name.format(),
                        space(),
                        in_token.format(),
                        space(),
                        keys_type.format(),
                        as_clause.as_ref().map(|_| space()),
                        as_clause.format(),
                        r_brack_token.format(),
                    ]),
                    optional_modifier.format(),
                    type_annotation_has_leading_comment.then_some(space()),
                    mapped_type.format(),
                    if_group_breaks(&FormatOptionalSemicolon::new(semicolon_token.as_ref()))
                ]
            )
        });

        let should_insert_space_around_brackets = f.options().bracket_spacing().value();
        write!(
            f,
            [
                &l_curly_token.format(),
                group(&soft_block_indent_with_maybe_space(
                    &format_inner,
                    should_insert_space_around_brackets
                ))
                .should_expand(should_expand),
                r_curly_token.format(),
            ]
        )
    }

    fn fmt_dangling_comments(&self, _: &TsMappedType, _: &mut JsFormatter) -> FormatResult<()> {
        // Handled inside of `fmt_fields`
        Ok(())
    }
}

/// Check if the user introduced a new line inside the node, but only if
/// that new line occurs at or before the property name. For example,
/// this would break:
///   { [
///     A in B]: T}
/// Because the line break occurs before `A`, the property name. But this
/// would _not_ break:
///   { [A
///     in B]: T}
/// Because the break is _after_ the `A`.
fn has_line_break_before_property_name(node: &TsMappedType) -> FormatResult<bool> {
    let property_name = node.property_name()?;
    let first_property_name_token = match property_name.syntax().first_token() {
        Some(first_token) => first_token,
        None => return Err(FormatError::SyntaxError),
    };

    let result = node
        .syntax()
        .descendants_tokens(Direction::Next)
        // Skip the first token to avoid formatter instability. See #4165.
        // This also makes sense since leading trivia of the first token
        // are not part of the interior of the node.
        .skip(1)
        // Only process up until the first token of the property name
        .take_while(|token| first_property_name_token != *token)
        .flat_map(|token| {
            token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces())
        })
        // Add only the leading trivia of the first property name token to
        // ensure that any newline in front of it is included. Otherwise
        // the `take_while` stops at the previous token, and the trailing
        // trivia won't include the newline.
        .chain(first_property_name_token.leading_trivia().pieces())
        .any(|piece| piece.is_newline());

    Ok(result)
}
