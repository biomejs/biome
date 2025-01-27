use crate::prelude::*;
use crate::utils::{FormatOptionalSemicolon, FormatStatementSemicolon};

use biome_formatter::{format_args, write, CstFormatContext};
use biome_js_syntax::binary_like_expression::AnyJsBinaryLikeExpression;
use biome_js_syntax::expression_left_side::AnyJsExpressionLeftSide;
use biome_js_syntax::{
    AnyJsExpression, JsReturnStatement, JsSequenceExpression, JsSyntaxToken, JsThrowStatement,
};
use biome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsReturnStatement;

impl FormatNodeRule<JsReturnStatement> for FormatJsReturnStatement {
    fn fmt_fields(&self, node: &JsReturnStatement, f: &mut JsFormatter) -> FormatResult<()> {
        AnyJsStatementWithArgument::from(node.clone()).fmt(f)
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsReturnStatement,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `JsAnyStatementWithArgument`
        Ok(())
    }
}

declare_node_union! {
    pub(super) AnyJsStatementWithArgument = JsThrowStatement | JsReturnStatement
}

impl Format<JsFormatContext> for AnyJsStatementWithArgument {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        write!(f, [self.operation_token().format()])?;

        let argument = self.argument()?;

        if let Some(semicolon) = self.semicolon_token() {
            if let Some(argument) = argument {
                write!(f, [space(), FormatReturnOrThrowArgument(&argument)])?;
            }

            let comments = f.context().comments();
            let has_dangling_comments = comments.has_dangling_comments(self.syntax());

            let is_last_comment_line = comments
                .trailing_comments(self.syntax())
                .last()
                .or_else(|| comments.dangling_comments(self.syntax()).last())
                .is_some_and(|comment| comment.kind().is_line());

            if is_last_comment_line {
                write!(f, [FormatOptionalSemicolon::new(Some(&semicolon))])?;
            }

            if has_dangling_comments {
                write!(f, [space(), format_dangling_comments(self.syntax())])?;
            }

            if !is_last_comment_line {
                write!(f, [FormatOptionalSemicolon::new(Some(&semicolon))])?;
            }

            Ok(())
        } else {
            if let Some(argument) = &argument {
                write!(f, [space(), FormatReturnOrThrowArgument(argument)])?;
            }

            write!(f, [FormatStatementSemicolon::new(None)])
        }
    }
}

impl AnyJsStatementWithArgument {
    fn operation_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsStatementWithArgument::JsThrowStatement(throw) => throw.throw_token(),
            AnyJsStatementWithArgument::JsReturnStatement(ret) => ret.return_token(),
        }
    }

    fn argument(&self) -> SyntaxResult<Option<AnyJsExpression>> {
        match self {
            AnyJsStatementWithArgument::JsThrowStatement(throw) => throw.argument().map(Some),
            AnyJsStatementWithArgument::JsReturnStatement(ret) => Ok(ret.argument()),
        }
    }

    fn semicolon_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsStatementWithArgument::JsThrowStatement(throw) => throw.semicolon_token(),
            AnyJsStatementWithArgument::JsReturnStatement(ret) => ret.semicolon_token(),
        }
    }
}

pub(super) struct FormatReturnOrThrowArgument<'a>(&'a AnyJsExpression);

impl Format<JsFormatContext> for FormatReturnOrThrowArgument<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let argument = self.0;
        let is_suppressed = f.comments().is_suppressed(argument.syntax());

        if has_argument_leading_comments(argument, f.context().comments())
            && !matches!(argument, AnyJsExpression::JsxTagExpression(_))
            && !is_suppressed
        {
            write!(f, [text("("), &block_indent(&argument.format()), text(")")])
        } else if is_binary_or_sequence_argument(argument) && !is_suppressed {
            write!(
                f,
                [group(&format_args![
                    if_group_breaks(&text("(")),
                    soft_block_indent(&argument.format()),
                    if_group_breaks(&text(")"))
                ])]
            )
        } else {
            write!(f, [argument.format()])
        }
    }
}

/// Tests if the passed in argument has any leading comments. This is the case if
/// * the argument has any leading comment
/// * the argument's left side has any leading comment (see [get_expression_left_side]).
///
/// Traversing the left nodes is necessary in case the first node is parenthesized because
/// parentheses will be removed (and be re-added by the return statement, but only if the argument breaks)
fn has_argument_leading_comments(argument: &AnyJsExpression, comments: &JsComments) -> bool {
    let mut current: Option<AnyJsExpressionLeftSide> = Some(argument.clone().into());

    while let Some(expression) = current {
        if comments.has_leading_own_line_comment(expression.syntax()) {
            return true;
        }

        if comments
            .leading_comments(argument.syntax())
            .iter()
            .any(|comment| comment.piece().has_newline())
        {
            return true;
        };

        current = expression.left_expression();
    }

    false
}

fn is_binary_or_sequence_argument(argument: &AnyJsExpression) -> bool {
    JsSequenceExpression::can_cast(argument.syntax().kind())
        || AnyJsBinaryLikeExpression::can_cast(argument.syntax().kind())
}
