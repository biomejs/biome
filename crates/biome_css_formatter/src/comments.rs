use crate::prelude::*;
use crate::utils::comment_trivia::is_trailing_comment_on_node;
use crate::utils::scss_include_comments::{
    place_list_trailing_separator_comment, place_map_trailing_separator_comment,
    place_separated_list_comment,
};
use biome_css_syntax::{
    AnyCssDeclarationName, AnyCssRoot, CssComplexSelector, CssDeclarationOrRuleBlock, CssFunction,
    CssGenericProperty, CssIdentifier, CssLanguage, CssSyntaxKind, ScssAtRootAtRule,
    ScssAtRootSelector, ScssEachHeader, ScssExpression, ScssExpressionItemList, ScssIfAtRule,
    ScssListExpression, ScssListExpressionElement, ScssMapExpression, ScssMapExpressionPair, T,
    TextLen, TextSize, is_in_scss_include_arguments, single_expression_item,
};
use biome_diagnostics::category;
use biome_formatter::comments::{
    CommentKind, CommentPlacement, CommentStyle, Comments, DecoratedComment, SourceComment,
    is_doc_comment,
};
use biome_formatter::formatter::Formatter;
use biome_formatter::{FormatResult, FormatRule, write};
use biome_rowan::SyntaxTriviaPieceComments;
use biome_suppression::{SuppressionKind, parse_suppression_comment};

pub type CssComments = Comments<CssLanguage>;

#[derive(Default)]
pub struct FormatCssLeadingComment;

impl FormatRule<SourceComment<CssLanguage>> for FormatCssLeadingComment {
    type Context = CssFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<CssLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        if is_doc_comment(comment.piece()) {
            let mut source_offset = comment.piece().text_range().start();

            let mut lines = comment.piece().text().lines();

            // SAFETY: Safe, `is_doc_comment` only returns `true` for multiline comments
            let first_line = lines.next().unwrap();
            write!(f, [text(first_line.trim_end(), source_offset)])?;

            source_offset += first_line.text_len();

            // Indent the remaining lines by one space so that all `*` are aligned.
            write!(
                f,
                [align(
                    1,
                    &format_once(|f| {
                        for line in lines {
                            write!(f, [hard_line_break(), text(line.trim(), source_offset)])?;

                            source_offset += line.text_len();
                        }

                        Ok(())
                    })
                )]
            )
        } else {
            write!(f, [comment.piece().as_piece()])
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct CssCommentStyle;

impl CommentStyle for CssCommentStyle {
    type Language = CssLanguage;

    fn is_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .filter(|suppression| suppression.kind == SuppressionKind::Classic)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, ..)| key == category!("format"))
    }

    fn is_global_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .filter(|suppression| suppression.kind == SuppressionKind::All)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, ..)| key == category!("format"))
    }

    fn get_comment_kind(comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
        if comment.text().starts_with("/*") {
            if comment.has_newline() {
                CommentKind::Block
            } else {
                CommentKind::InlineBlock
            }
        } else {
            CommentKind::Line
        }
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        handle_scss_map_trailing_separator_comment(comment)
            .or_else(place_separated_list_comment)
            .or_else(handle_scss_list_trailing_separator_comment)
            .or_else(handle_scss_each_iterable_comment)
            .or_else(handle_scss_expression_item_trailing_line_comment)
            .or_else(handle_scss_at_root_selector_comment)
            .or_else(handle_scss_else_clause_comment)
            .or_else(handle_function_comment)
            .or_else(handle_generic_property_comment)
            .or_else(handle_declaration_name_comment)
            .or_else(handle_complex_selector_comment)
            .or_else(handle_global_suppression)
    }
}

fn handle_scss_map_trailing_separator_comment(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    let Some(preceding_pair) = comment
        .preceding_node()
        .and_then(ScssMapExpressionPair::cast_ref)
    else {
        return CommentPlacement::Default(comment);
    };

    let Some(map_expression) =
        ScssMapExpression::cast_ref(comment.enclosing_node()).or_else(|| {
            preceding_pair
                .syntax()
                .ancestors()
                .find_map(ScssMapExpression::cast)
        })
    else {
        return CommentPlacement::Default(comment);
    };

    place_map_trailing_separator_comment(&map_expression, &preceding_pair, comment)
}

fn handle_scss_list_trailing_separator_comment(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    let Some(preceding_element) = comment
        .preceding_node()
        .and_then(ScssListExpressionElement::cast_ref)
    else {
        return CommentPlacement::Default(comment);
    };

    let Some(list_expression) = preceding_element
        .syntax()
        .ancestors()
        .find_map(ScssListExpression::cast)
    else {
        return CommentPlacement::Default(comment);
    };

    place_list_trailing_separator_comment(&list_expression, &preceding_element, comment)
}

/// Keeps `@each ... in /* comment */ (a, b)` comments with the iterable.
fn handle_scss_each_iterable_comment(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    let Some(expression) = comment.following_node().and_then(ScssExpression::cast_ref) else {
        return CommentPlacement::Default(comment);
    };

    if !expression
        .syntax()
        .parent()
        .is_some_and(|parent| ScssEachHeader::can_cast(parent.kind()))
    {
        return CommentPlacement::Default(comment);
    }

    if !single_expression_item(&expression).is_some_and(|item| {
        item.as_scss_list_expression().is_some() || item.as_scss_map_expression().is_some()
    }) {
        return CommentPlacement::Default(comment);
    }

    CommentPlacement::leading(expression.into_syntax(), comment)
}

fn handle_scss_expression_item_trailing_line_comment(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    let (Some(preceding_node), Some(following_node)) =
        (comment.preceding_node(), comment.following_node())
    else {
        return CommentPlacement::Default(comment);
    };

    if !comment.kind().is_line()
        || !comment.text_position().is_end_of_line()
        || !is_trailing_comment_on_node(preceding_node, &comment)
        || is_in_scss_include_arguments(preceding_node)
    {
        return CommentPlacement::Default(comment);
    }

    let Some(list) = preceding_node
        .parent()
        .filter(|parent| ScssExpressionItemList::can_cast(parent.kind()))
    else {
        return CommentPlacement::Default(comment);
    };

    if following_node.parent().as_ref() == Some(&list) {
        CommentPlacement::trailing(preceding_node.clone(), comment)
    } else {
        CommentPlacement::Default(comment)
    }
}

fn handle_scss_at_root_selector_comment(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    // Keep selector comments before `{`, matching Prettier's `@at-root` output.
    if !comment.kind().is_line()
        || !comment.text_position().is_own_line()
        || comment
            .following_token()
            .is_none_or(|token| token.kind() != T!['{'])
    {
        return CommentPlacement::Default(comment);
    }

    let Some(selector) = comment
        .preceding_node()
        .and_then(ScssAtRootSelector::cast_ref)
    else {
        return CommentPlacement::Default(comment);
    };

    let Some(at_root) = selector.syntax().parent().and_then(ScssAtRootAtRule::cast) else {
        return CommentPlacement::Default(comment);
    };

    match at_root.block() {
        Ok(block) => CommentPlacement::leading(block.into_syntax(), comment),
        Err(_) => CommentPlacement::Default(comment),
    }
}

fn handle_scss_else_clause_comment(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    let Some(following_token_range) = comment
        .following_token()
        .filter(|token| token.kind() == T![@])
        .map(|token| token.text_range())
    else {
        return CommentPlacement::Default(comment);
    };

    if !comment.kind().is_line() || !comment.text_position().is_own_line() {
        return CommentPlacement::Default(comment);
    }

    let Some(block) = comment
        .preceding_node()
        .and_then(CssDeclarationOrRuleBlock::cast_ref)
    else {
        return CommentPlacement::Default(comment);
    };

    let Some(else_clause) = block
        .syntax()
        .parent()
        .and_then(ScssIfAtRule::cast)
        .and_then(|if_rule| if_rule.else_clause())
    else {
        return CommentPlacement::Default(comment);
    };

    match else_clause.at_token() {
        Ok(at_token) if at_token.text_range() == following_token_range => {
            CommentPlacement::leading(else_clause.into_syntax(), comment)
        }
        _ => CommentPlacement::Default(comment),
    }
}

fn handle_function_comment(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    let (Some(preceding_node), Some(following_node)) =
        (comment.preceding_node(), comment.following_node())
    else {
        return CommentPlacement::Default(comment);
    };

    let is_inside_function = CssFunction::can_cast(comment.enclosing_node().kind());
    let is_after_name = CssIdentifier::can_cast(preceding_node.kind());
    if is_inside_function && is_after_name {
        CommentPlacement::leading(following_node.clone(), comment)
    } else {
        CommentPlacement::Default(comment)
    }
}

fn handle_generic_property_comment(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    // Check if the comment is inside a CSS generic property (e.g., color: value)
    let Some(generic_property) = comment
        .enclosing_node()
        .ancestors()
        .find_map(CssGenericProperty::cast)
    else {
        return CommentPlacement::Default(comment);
    };

    let Ok(name) = generic_property.name() else {
        return CommentPlacement::Default(comment);
    };

    let comment_piece = comment.piece();

    // Check if the comment is in the name's trailing trivia (before colon)
    // Example: `color /* comment */: value`
    if let Some(name_token) = name.syntax().last_token() {
        for piece in name_token.trailing_trivia().pieces() {
            if piece.is_comments() && piece.text() == comment_piece.text() {
                // Our placement is slightly better than Prettier because it adds some spacing
                return CommentPlacement::trailing(name.into_syntax(), comment);
            }
        }
    }

    if let (Some(preceding), Some(following)) = (comment.preceding_node(), comment.following_node())
    {
        // If preceding is the property name and following is in the value list
        if preceding == name.syntax()
            && following
                .parent()
                .is_some_and(|p| p.kind() == CssSyntaxKind::CSS_GENERIC_COMPONENT_VALUE_LIST)
        {
            // Place comment as dangling on the property so it can be formatted inline
            // between the colon and values
            return CommentPlacement::trailing(generic_property.into_syntax(), comment);
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_declaration_name_comment(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    match comment.preceding_node() {
        Some(following_node) if AnyCssDeclarationName::can_cast(following_node.kind()) => {
            if following_node
                .parent()
                .is_some_and(|p| p.kind() == CssSyntaxKind::CSS_GENERIC_COMPONENT_VALUE_LIST)
            {
                CommentPlacement::Default(comment)
            } else {
                CommentPlacement::leading(following_node.clone(), comment)
            }
        }
        _ => CommentPlacement::Default(comment),
    }
}

fn handle_complex_selector_comment(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    if let Some(complex) = CssComplexSelector::cast_ref(comment.enclosing_node())
        && let Ok(right) = complex.right()
    {
        return CommentPlacement::leading(right.into_syntax(), comment);
    }
    CommentPlacement::Default(comment)
}

fn handle_global_suppression(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    let node = comment.enclosing_node();

    if node.text_range_with_trivia().start() == TextSize::from(0) {
        let has_global_suppression = node.first_leading_trivia().is_some_and(|trivia| {
            trivia
                .pieces()
                .filter(|piece| piece.is_comments())
                .any(|piece| CssCommentStyle::is_global_suppression(piece.text()))
        });
        let root = node.ancestors().find_map(AnyCssRoot::cast);
        if let Some(root) = root
            && has_global_suppression
        {
            return CommentPlacement::leading(root.syntax().clone(), comment);
        }
    }

    CommentPlacement::Default(comment)
}
