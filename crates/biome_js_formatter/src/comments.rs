use crate::prelude::*;
use crate::utils::AnyJsConditional;
use biome_diagnostics_categories::category;
use biome_formatter::comments::is_alignable_comment;
use biome_formatter::{
    comments::{
        CommentKind, CommentPlacement, CommentStyle, CommentTextPosition, Comments,
        DecoratedComment, SourceComment,
    },
    write,
};
use biome_js_syntax::JsSyntaxKind::JS_EXPORT;
use biome_js_syntax::{
    AnyJsClass, AnyJsName, AnyJsRoot, AnyJsStatement, JsArrayHole, JsArrowFunctionExpression,
    JsBlockStatement, JsCallArguments, JsCatchClause, JsEmptyStatement, JsFinallyClause,
    JsFormalParameter, JsFunctionBody, JsIdentifierBinding, JsIdentifierExpression, JsIfStatement,
    JsLanguage, JsNamedImportSpecifiers, JsParameters, JsSyntaxKind, JsSyntaxNode,
    JsVariableDeclarator, JsWhileStatement, TsInterfaceDeclaration, TsMappedType,
};
use biome_rowan::{AstNode, SyntaxNodeOptionExt, SyntaxTriviaPieceComments, TextLen};
use biome_suppression::parse_suppression_comment;

pub type JsComments = Comments<JsLanguage>;

#[derive(Default)]
pub struct FormatJsLeadingComment;

impl FormatRule<SourceComment<JsLanguage>> for FormatJsLeadingComment {
    type Context = JsFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<JsLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        if is_alignable_comment(comment.piece()) {
            let mut source_offset = comment.piece().text_range().start();

            let mut lines = comment.piece().text().lines();

            // SAFETY: Safe, `is_alignable_comment` only returns `true` for multiline comments
            let first_line = lines.next().unwrap();
            write!(f, [dynamic_text(first_line.trim_end(), source_offset)])?;

            source_offset += first_line.text_len();

            // Indent the remaining lines by one space so that all `*` are aligned.
            write!(
                f,
                [&format_once(|f| {
                    for line in lines {
                        write!(
                            f,
                            [
                                hard_line_break(),
                                text(" "),
                                dynamic_text(line.trim(), source_offset)
                            ]
                        )?;

                        source_offset += line.text_len();
                    }

                    Ok(())
                })]
            )
        } else {
            write!(f, [comment.piece().as_piece()])
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct JsCommentStyle;

impl CommentStyle for JsCommentStyle {
    type Language = JsLanguage;

    fn is_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, _)| key == category!("format"))
    }

    fn get_comment_kind(comment: &SyntaxTriviaPieceComments<JsLanguage>) -> CommentKind {
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
        match comment.text_position() {
            CommentTextPosition::EndOfLine => handle_typecast_comment(comment)
                .or_else(handle_function_comment)
                .or_else(handle_conditional_comment)
                .or_else(handle_if_statement_comment)
                .or_else(handle_while_comment)
                .or_else(handle_try_comment)
                .or_else(handle_class_comment)
                .or_else(handle_method_comment)
                .or_else(handle_for_comment)
                .or_else(handle_root_comments)
                .or_else(handle_variable_declarator_comment)
                .or_else(handle_parameter_comment)
                .or_else(handle_labelled_statement_comment)
                .or_else(handle_call_expression_comment)
                .or_else(handle_continue_break_comment)
                .or_else(handle_mapped_type_comment)
                .or_else(handle_switch_default_case_comment)
                .or_else(handle_after_arrow_fat_arrow_comment)
                .or_else(handle_import_export_specifier_comment)
                .or_else(handle_import_named_clause_comments)
                .or_else(handle_array_expression),
            CommentTextPosition::OwnLine => handle_member_expression_comment(comment)
                .or_else(handle_function_comment)
                .or_else(handle_if_statement_comment)
                .or_else(handle_while_comment)
                .or_else(handle_try_comment)
                .or_else(handle_class_comment)
                .or_else(handle_method_comment)
                .or_else(handle_for_comment)
                .or_else(handle_root_comments)
                .or_else(handle_parameter_comment)
                .or_else(handle_labelled_statement_comment)
                .or_else(handle_call_expression_comment)
                .or_else(handle_mapped_type_comment)
                .or_else(handle_continue_break_comment)
                .or_else(handle_union_type_comment)
                .or_else(handle_import_export_specifier_comment)
                .or_else(handle_class_method_comment)
                .or_else(handle_import_named_clause_comments),
            CommentTextPosition::SameLine => handle_if_statement_comment(comment)
                .or_else(handle_while_comment)
                .or_else(handle_for_comment)
                .or_else(handle_root_comments)
                .or_else(handle_after_arrow_param_comment)
                .or_else(handle_array_hole_comment)
                .or_else(handle_call_expression_comment)
                .or_else(handle_continue_break_comment)
                .or_else(handle_class_comment)
                .or_else(handle_declare_comment)
                .or_else(handle_import_named_clause_comments),
        }
    }
}

/// Force end of line type cast comments to remain leading comments of the next node, if any
fn handle_typecast_comment(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
    match comment.following_node() {
        Some(following_node) if is_type_comment(comment.piece()) => {
            CommentPlacement::leading(following_node.clone(), comment)
        }
        _ => CommentPlacement::Default(comment),
    }
}

/// Move the arrow function's comment to the same position as the prettier
fn handle_after_arrow_fat_arrow_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    if JsArrowFunctionExpression::can_cast(comment.enclosing_node().kind()) {
        // input
        // ```javascript
        // num => // comment
        // c;
        // ```
        // output
        // ```javascript
        //(
        //   num, // comment
        // ) => c;
        // ```
        if let Some(js_ident_binding) = comment
            .preceding_node()
            .and_then(JsIdentifierBinding::cast_ref)
        {
            return CommentPlacement::trailing(js_ident_binding.into_syntax(), comment);
        }
        // input
        // ```javascript
        // (num1,num2) => // comment
        // c;
        // ```
        // output
        // ```javascript
        // (
        //     num1,
        //     num2, // comment
        //  ) => c;
        // ```
        if let Some(js_parameters) = comment.preceding_node().and_then(JsParameters::cast_ref) {
            if let Some(Ok(last)) = js_parameters.items().last() {
                return CommentPlacement::trailing(last.into_syntax(), comment);
            };
        }
        // input
        // ```javascript
        // () => // comment
        // c;
        // ```
        // output
        // ```javascript
        // () =>
        // // comment
        // c;
        // ```
        if let Some(following_node) = comment.following_node() {
            return CommentPlacement::leading(following_node.clone(), comment);
        }
    }
    CommentPlacement::Default(comment)
}

fn handle_after_arrow_param_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    let is_next_arrow = comment
        .following_token()
        .is_some_and(|token| token.kind() == JsSyntaxKind::FAT_ARROW);

    // Makes comments after the `(` and `=>` dangling comments
    // ```javascript
    // () /* comment */ => true
    // ```
    if JsArrowFunctionExpression::can_cast(comment.enclosing_node().kind()) && is_next_arrow {
        CommentPlacement::dangling(comment.enclosing_node().clone(), comment)
    } else {
        CommentPlacement::Default(comment)
    }
}

/// Handles array hole comments. Array holes have no token so all comments
/// become trailing comments by default. Override it that all comments are leading comments.
fn handle_array_hole_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    if let Some(array_hole) = comment.preceding_node().and_then(JsArrayHole::cast_ref) {
        CommentPlacement::leading(array_hole.into_syntax(), comment)
    } else {
        CommentPlacement::Default(comment)
    }
}

fn handle_call_expression_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    // Make comments between the callee and the arguments leading comments of the first argument.
    // ```javascript
    // a /* comment */ (call)
    // ```
    if let Some(arguments) = comment.following_node().and_then(JsCallArguments::cast_ref) {
        return if let Some(Ok(first)) = arguments.args().first() {
            CommentPlacement::leading(first.into_syntax(), comment)
        } else {
            CommentPlacement::dangling(arguments.into_syntax(), comment)
        };
    }

    CommentPlacement::Default(comment)
}

fn handle_continue_break_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    // Make comments between `continue`/`break` and the label, leading comments of the label.
    // ```javascript
    // continue /* comment */ a;
    // ```
    if let Some(next) = comment.following_node() {
        if next.kind() == JsSyntaxKind::JS_LABEL {
            return CommentPlacement::leading(next.clone(), comment);
        }
    }

    let enclosing = comment.enclosing_node();

    if let (Some(preceding), Some(parent)) =
        (comment.preceding_node(), comment.enclosing_node().parent())
    {
        if preceding.kind() == JsSyntaxKind::JS_LABEL
            && parent.kind() != JsSyntaxKind::JS_FOR_STATEMENT
        {
            return CommentPlacement::trailing(preceding.clone(), comment);
        }
    }

    // Make comments between the `continue` and label token trailing comments
    // ```javascript
    // continue /* comment */ a;
    // ```
    // This differs from Prettier because other ASTs use an identifier for the label whereas Biome uses
    // a token.
    match enclosing.kind() {
        JsSyntaxKind::JS_CONTINUE_STATEMENT | JsSyntaxKind::JS_BREAK_STATEMENT => {
            match enclosing.parent() {
                // Make it the trailing of the parent if this is a single-statement body
                // to prevent that the comment becomes a trailing comment of the parent when re-formatting
                // ```javascript
                // for (;;) continue /* comment */;
                // ```
                Some(parent)
                    if matches!(
                        parent.kind(),
                        JsSyntaxKind::JS_FOR_STATEMENT
                            | JsSyntaxKind::JS_FOR_OF_STATEMENT
                            | JsSyntaxKind::JS_FOR_IN_STATEMENT
                            | JsSyntaxKind::JS_WHILE_STATEMENT
                            | JsSyntaxKind::JS_DO_WHILE_STATEMENT
                            | JsSyntaxKind::JS_IF_STATEMENT
                            | JsSyntaxKind::JS_WITH_STATEMENT
                            | JsSyntaxKind::JS_LABELED_STATEMENT
                    ) =>
                {
                    CommentPlacement::trailing(parent, comment)
                }
                _ => CommentPlacement::trailing(enclosing.clone(), comment),
            }
        }
        _ => CommentPlacement::Default(comment),
    }
}

/// Moves line comments after the `default` keyword into the block statement:
///
/// ```javascript
/// switch (x) {
///     default: // comment
///     {
///         break;
///     }
/// ```
///
/// All other same line comments will use `Default` placement if they have a preceding node.
/// ```javascript
/// switch(x) {
///     default:
///         a(); // asd
///         break;
/// }
/// ```
///
/// All other comments become `Dangling` comments that are handled inside of the default case
/// formatting.
fn handle_switch_default_case_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    if comment.enclosing_node().kind() != JsSyntaxKind::JS_DEFAULT_CLAUSE {
        return CommentPlacement::Default(comment);
    }

    if !comment.kind().is_line() {
        return CommentPlacement::dangling(comment.enclosing_node().clone(), comment);
    }

    let Some(block) = comment
        .following_node()
        .and_then(JsBlockStatement::cast_ref)
    else {
        if comment.preceding_node().is_some() {
            return CommentPlacement::Default(comment);
        }
        return CommentPlacement::dangling(comment.enclosing_node().clone(), comment);
    };

    place_block_statement_comment(block, comment)
}

fn handle_labelled_statement_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    match comment.enclosing_node().kind() {
        JsSyntaxKind::JS_LABELED_STATEMENT => {
            CommentPlacement::leading(comment.enclosing_node().clone(), comment)
        }
        _ => CommentPlacement::Default(comment),
    }
}

/// Moves comments in declaration statements to behind the identifier
///
/// ```javascript
/// declare module /* comment */ A {}
/// declare /* module */ global {}
/// ```
fn handle_declare_comment(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
    match (comment.enclosing_node().kind(), comment.following_node()) {
        // Check if it is a declare statement
        (JsSyntaxKind::TS_DECLARE_STATEMENT, Some(following)) => {
            match following.kind() {
                JsSyntaxKind::TS_GLOBAL_DECLARATION => {
                    // Global declarations have no identifier, so keep at default
                    CommentPlacement::Default(comment)
                }
                JsSyntaxKind::TS_MODULE_DECLARATION
                | JsSyntaxKind::TS_ENUM_DECLARATION
                | JsSyntaxKind::TS_INTERFACE_DECLARATION
                | JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION
                | JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION => {
                    // Move comment after the module keyword
                    // This is the first child of the module declaration which is the identifier
                    if let Some(first_child) = following.first_child() {
                        return CommentPlacement::leading(first_child.clone(), comment);
                    }
                    CommentPlacement::Default(comment)
                }
                JsSyntaxKind::JS_CLASS_DECLARATION => {
                    if let Some(first_child) = following.first_child() {
                        if let Some(second_child) = first_child.next_sibling() {
                            return CommentPlacement::leading(second_child.clone(), comment);
                        }
                    }
                    CommentPlacement::Default(comment)
                }
                JsSyntaxKind::JS_VARIABLE_DECLARATION_CLAUSE => {
                    let first_identifier = following
                        .descendants()
                        .find(|node| node.kind() == JsSyntaxKind::JS_IDENTIFIER_BINDING);
                    if let Some(first_identifier_exists) = first_identifier {
                        return CommentPlacement::leading(first_identifier_exists.clone(), comment);
                    }
                    CommentPlacement::Default(comment)
                }
                _ => CommentPlacement::Default(comment),
            }
        }
        _ => CommentPlacement::Default(comment),
    }
}

fn handle_class_comment(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
    // Make comments following after the `extends` or `implements` keyword trailing comments
    // of the preceding extends, type parameters, or id.
    // ```javascript
    // class a9 extends
    //   /* comment */
    // b {
    //   constructor() {}
    // }
    // ```
    if matches!(
        comment.enclosing_node().kind(),
        JsSyntaxKind::JS_EXTENDS_CLAUSE
            | JsSyntaxKind::TS_IMPLEMENTS_CLAUSE
            | JsSyntaxKind::TS_EXTENDS_CLAUSE
    ) {
        if comment.preceding_node().is_none() && !comment.text_position().is_same_line() {
            if let Some(sibling) = comment.enclosing_node().prev_sibling() {
                return CommentPlacement::trailing(sibling, comment);
            }
        }

        return CommentPlacement::Default(comment);
    }

    // ```javascript
    // @decorator
    // // comment
    // class Foo {}
    // ```
    if (AnyJsClass::can_cast(comment.enclosing_node().kind())
        && comment
            .following_token()
            .is_some_and(|token| token.kind() == JsSyntaxKind::CLASS_KW))
        // ```javascript
        // @decorator
        // // comment
        // export class Foo {}
        // ```
        || comment.enclosing_node().kind() == JS_EXPORT
    {
        if let Some(preceding) = comment.preceding_node() {
            if preceding.kind() == JsSyntaxKind::JS_DECORATOR {
                return CommentPlacement::trailing(preceding.clone(), comment);
            }
        }
    }

    let first_member = if let Some(class) = AnyJsClass::cast_ref(comment.enclosing_node()) {
        class.members().first().map(AstNode::into_syntax)
    } else if let Some(interface) = TsInterfaceDeclaration::cast_ref(comment.enclosing_node()) {
        interface.members().first().map(AstNode::into_syntax)
    } else {
        return CommentPlacement::Default(comment);
    };

    if comment.text_position().is_same_line() {
        // Handle same line comments in empty class bodies
        // ```javascript
        // class Test { /* comment */ }
        // ```
        if comment
            .following_token()
            .is_some_and(|token| token.kind() == JsSyntaxKind::R_CURLY)
            && first_member.is_none()
        {
            return CommentPlacement::dangling(comment.enclosing_node().clone(), comment);
        } else {
            return CommentPlacement::Default(comment);
        }
    }

    if let Some(following) = comment.following_node() {
        // Make comments preceding the first member leading comments of the member
        // ```javascript
        // class Test { /* comment */
        //      prop;
        // }
        // ```
        if let Some(member) = first_member {
            if following == &member {
                return CommentPlacement::leading(member, comment);
            }
        }

        if let Some(preceding) = comment.preceding_node() {
            // Make all comments between the id/type parameters and the extends clause trailing comments
            // of the id/type parameters
            //
            // ```javascript
            // class Test
            //
            // /* comment */ extends B {}
            // ```
            if matches!(
                following.kind(),
                JsSyntaxKind::JS_EXTENDS_CLAUSE
                    | JsSyntaxKind::TS_IMPLEMENTS_CLAUSE
                    | JsSyntaxKind::TS_EXTENDS_CLAUSE
            ) && !comment.text_position().is_same_line()
            {
                return CommentPlacement::trailing(preceding.clone(), comment);
            }
        }
    } else if first_member.is_none() {
        // Handle the case where there are no members, attach the comments as dangling comments.
        // ```javascript
        // class Test // comment
        // {
        // }
        // ```
        return CommentPlacement::dangling(comment.enclosing_node().clone(), comment);
    }

    CommentPlacement::Default(comment)
}

fn handle_method_comment(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
    let enclosing = comment.enclosing_node();

    let is_method = matches!(
        enclosing.kind(),
        JsSyntaxKind::JS_METHOD_CLASS_MEMBER
            | JsSyntaxKind::JS_METHOD_OBJECT_MEMBER
            | JsSyntaxKind::JS_SETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_SETTER_OBJECT_MEMBER
            | JsSyntaxKind::JS_GETTER_OBJECT_MEMBER
            | JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER
    );

    if !is_method {
        return CommentPlacement::Default(comment);
    }

    // Move end of line and own line comments before the method body into the function
    // ```javascript
    // class Test {
    //   method() /* test */
    //  {}
    // }
    // ```
    if let Some(following) = comment.following_node() {
        if let Some(body) = JsFunctionBody::cast_ref(following) {
            if let Some(directive) = body.directives().first() {
                return CommentPlacement::leading(directive.into_syntax(), comment);
            }

            let first_non_empty = body
                .statements()
                .iter()
                .find(|statement| !matches!(statement, AnyJsStatement::JsEmptyStatement(_)));

            return match first_non_empty {
                None => CommentPlacement::dangling(body.into_syntax(), comment),
                Some(statement) => CommentPlacement::leading(statement.into_syntax(), comment),
            };
        }
    }

    CommentPlacement::Default(comment)
}

/// Handle a all comments document.
/// See `blank.js`
fn handle_root_comments(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
    if let Some(root) = AnyJsRoot::cast_ref(comment.enclosing_node()) {
        let is_blank = match &root {
            AnyJsRoot::JsExpressionSnipped(_) => false,
            AnyJsRoot::JsModule(module) => {
                module.directives().is_empty() && module.items().is_empty()
            }
            AnyJsRoot::JsScript(script) => {
                script.directives().is_empty() && script.statements().is_empty()
            }
            AnyJsRoot::TsDeclarationModule(module) => {
                module.directives().is_empty() && module.items().is_empty()
            }
        };

        if is_blank {
            return CommentPlacement::leading(root.into_syntax(), comment);
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_member_expression_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    let following = match comment.following_node() {
        Some(following)
            if matches!(
                comment.enclosing_node().kind(),
                JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                    | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION
            ) =>
        {
            following
        }
        _ => return CommentPlacement::Default(comment),
    };

    // ```javascript
    // a
    // /* comment */.b
    // a
    // /* comment */[b]
    // ```
    if AnyJsName::can_cast(following.kind()) || JsIdentifierExpression::can_cast(following.kind()) {
        CommentPlacement::leading(comment.enclosing_node().clone(), comment)
    } else {
        CommentPlacement::Default(comment)
    }
}

fn handle_function_comment(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
    if !matches!(
        comment.enclosing_node().kind(),
        JsSyntaxKind::JS_FUNCTION_DECLARATION
            | JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION
            | JsSyntaxKind::JS_FUNCTION_EXPRESSION
    ) || !comment.kind().is_line()
    {
        return CommentPlacement::Default(comment);
    };

    let Some(body) = comment.following_node().and_then(JsFunctionBody::cast_ref) else {
        return CommentPlacement::Default(comment);
    };

    // Make line comments between the `)` token and the function body leading comments
    // of the first statement or dangling comments of the body.
    // ```javascript
    // function test() // comment
    // {
    //  console.log("Hy");
    // }
    // ```
    match body.statements().first() {
        Some(first) => CommentPlacement::leading(first.into_syntax(), comment),
        None => CommentPlacement::dangling(body.into_syntax(), comment),
    }
}

fn handle_conditional_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    let enclosing = comment.enclosing_node();

    let (conditional, following) = match (
        AnyJsConditional::cast_ref(enclosing),
        comment.following_node(),
    ) {
        (Some(conditional), Some(following)) => (conditional, following),
        _ => {
            return CommentPlacement::Default(comment);
        }
    };

    // Make end of line comments that come after the operator leading comments of the consequent / alternate.
    // ```javascript
    // a
    //   // becomes leading of consequent
    //   ? { x: 5 } :
    //   {};
    //
    // a
    //   ? { x: 5 }
    //   : // becomes leading of alternate
    // 	{};
    //
    // a // remains trailing, because it directly follows the node
    //   ? { x: 5 }
    //   : {};
    // ```
    let token = comment.piece().as_piece().token();
    let is_after_operator = conditional.colon_token().as_ref() == Ok(&token)
        || conditional.question_mark_token().as_ref() == Ok(&token);

    if is_after_operator {
        return CommentPlacement::leading(following.clone(), comment);
    }

    CommentPlacement::Default(comment)
}

fn handle_if_statement_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    fn handle_else_clause(
        comment: DecoratedComment<JsLanguage>,
        consequent: JsSyntaxNode,
        if_statement: JsSyntaxNode,
    ) -> CommentPlacement<JsLanguage> {
        // Make all comments trailing comments of the `consequent` if the `consequent` is a `JsBlockStatement`
        // ```javascript
        // if (test) {
        //
        // } /* comment */ else if (b) {
        //     test
        // }
        // /* comment */ else if(c) {
        //
        // } /*comment */ else {
        //
        // }
        // ```
        if consequent.kind() == JsSyntaxKind::JS_BLOCK_STATEMENT {
            return CommentPlacement::trailing(consequent, comment);
        }

        // Handle end of line comments that aren't stretching over multiple lines.
        // Make them dangling comments of the consequent expression
        //
        // ```javascript
        // if (cond1) expr1; // comment A
        // else if (cond2) expr2; // comment A
        // else expr3;
        //
        // if (cond1) expr1; /* comment */ else  expr2;
        //
        // if (cond1) expr1; /* b */
        // else if (cond2) expr2; /* b */
        // else expr3; /* b*/
        // ```
        if !comment.kind().is_block()
            && !comment.text_position().is_own_line()
            && comment.preceding_node().is_some()
        {
            return CommentPlacement::trailing(consequent, comment);
        }

        // ```javascript
        // if (cond1) expr1;
        //
        // /* comment */ else  expr2;
        //
        // if (cond) expr; /*
        // a multiline comment */
        // else b;
        //
        // if (6) // comment
        // true
        // else // comment
        // {true}
        // ```
        CommentPlacement::dangling(if_statement, comment)
    }

    match (comment.enclosing_node().kind(), comment.following_node()) {
        (JsSyntaxKind::JS_IF_STATEMENT, Some(following)) => {
            let if_statement = JsIfStatement::unwrap_cast(comment.enclosing_node().clone());

            if let Some(preceding) = comment.preceding_node() {
                // Test if this is a comment right before the condition's `)`
                if comment
                    .following_token()
                    .is_some_and(|token| token.kind() == JsSyntaxKind::R_PAREN)
                {
                    return CommentPlacement::trailing(preceding.clone(), comment);
                }

                // Handle comments before `else`
                if following.kind() == JsSyntaxKind::JS_ELSE_CLAUSE {
                    let consequent = preceding.clone();
                    let if_statement = comment.enclosing_node().clone();
                    return handle_else_clause(comment, consequent, if_statement);
                }
            }

            // Move comments coming before the `{` inside of the block
            //
            // ```javascript
            // if (cond) /* test */ {
            // }
            // ```
            if let Some(block_statement) = JsBlockStatement::cast_ref(following) {
                return place_block_statement_comment(block_statement, comment);
            }

            // Don't attach comments to empty statements
            // ```javascript
            // if (cond) /* test */ ;
            // ```
            if let Some(preceding) = comment.preceding_node() {
                if JsEmptyStatement::can_cast(following.kind()) {
                    return CommentPlacement::trailing(preceding.clone(), comment);
                }
            }

            // Move comments coming before an if chain inside the body of the first non chain if.
            //
            // ```javascript
            // if (cond1)  /* test */ if (other) { a }
            // ```
            if let Some(if_statement) = JsIfStatement::cast_ref(following) {
                if let Ok(nested_consequent) = if_statement.consequent() {
                    return place_leading_statement_comment(nested_consequent, comment);
                }
            }

            // Make all comments after the condition's `)` leading comments
            // ```javascript
            // if (5) // comment
            // true
            //
            // ```
            if let Ok(consequent) = if_statement.consequent() {
                if consequent.syntax() == following {
                    return CommentPlacement::leading(following.clone(), comment);
                }
            }
        }
        (JsSyntaxKind::JS_ELSE_CLAUSE, _) => {
            if let Some(if_statement) = comment
                .enclosing_node()
                .parent()
                .and_then(JsIfStatement::cast)
            {
                if let Ok(consequent) = if_statement.consequent() {
                    return handle_else_clause(
                        comment,
                        consequent.into_syntax(),
                        if_statement.into_syntax(),
                    );
                }
            }
        }
        _ => {
            // fall through
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_while_comment(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
    let (while_statement, following) = match (
        JsWhileStatement::cast_ref(comment.enclosing_node()),
        comment.following_node(),
    ) {
        (Some(while_statement), Some(following)) => (while_statement, following),
        _ => return CommentPlacement::Default(comment),
    };

    if let Some(preceding) = comment.preceding_node() {
        // Test if this is a comment right before the condition's `)`
        if comment
            .following_token()
            .is_some_and(|token| token.kind() == JsSyntaxKind::R_PAREN)
        {
            return CommentPlacement::trailing(preceding.clone(), comment);
        }
    }

    // Move comments coming before the `{` inside of the block
    //
    // ```javascript
    // while (cond) /* test */ {
    // }
    // ```
    if let Some(block) = JsBlockStatement::cast_ref(following) {
        return place_block_statement_comment(block, comment);
    }

    // Don't attach comments to empty statements
    // ```javascript
    // if (cond) /* test */ ;
    // ```
    if let Some(preceding) = comment.preceding_node() {
        if JsEmptyStatement::can_cast(following.kind()) {
            return CommentPlacement::trailing(preceding.clone(), comment);
        }
    }

    // Make all comments after the condition's `)` leading comments
    // ```javascript
    // while (5) // comment
    // true
    //
    // ```
    if let Ok(body) = while_statement.body() {
        if body.syntax() == following {
            return CommentPlacement::leading(body.into_syntax(), comment);
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_try_comment(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
    let following = match comment.following_node() {
        Some(following)
            if matches!(
                comment.enclosing_node().kind(),
                JsSyntaxKind::JS_TRY_STATEMENT | JsSyntaxKind::JS_TRY_FINALLY_STATEMENT
            ) =>
        {
            // Move comments before the `catch` or `finally` inside of the body
            // ```javascript
            // try {
            // }
            //  catch(e) {
            // }
            // // Comment 7
            // finally {}
            // ```
            let body = if let Some(catch) = JsCatchClause::cast_ref(following) {
                catch.body()
            } else if let Some(finally) = JsFinallyClause::cast_ref(following) {
                finally.body()
            } else {
                // Use an err, so that the following code skips over it
                Err(biome_rowan::SyntaxError::MissingRequiredChild)
            };

            //
            // ```javascript
            // try {
            // } /* comment catch {
            // }
            // ```
            if let Ok(body) = body {
                return place_block_statement_comment(body, comment);
            }

            following
        }
        Some(following)
            if matches!(
                comment.enclosing_node().kind(),
                JsSyntaxKind::JS_CATCH_CLAUSE | JsSyntaxKind::JS_FINALLY_CLAUSE
            ) =>
        {
            following
        }
        _ => return CommentPlacement::Default(comment),
    };

    // Move comments coming before the `{` inside of the block
    //
    // ```javascript
    // try /* test */ {
    // }
    // ```
    if let Some(block) = JsBlockStatement::cast_ref(following) {
        return place_block_statement_comment(block, comment);
    }

    CommentPlacement::Default(comment)
}

fn handle_for_comment(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
    let enclosing = comment.enclosing_node();

    let is_for_in_or_of = matches!(
        enclosing.kind(),
        JsSyntaxKind::JS_FOR_OF_STATEMENT | JsSyntaxKind::JS_FOR_IN_STATEMENT
    );

    if !is_for_in_or_of && !matches!(enclosing.kind(), JsSyntaxKind::JS_FOR_STATEMENT) {
        return CommentPlacement::Default(comment);
    }

    if comment.text_position().is_own_line() && is_for_in_or_of {
        CommentPlacement::leading(enclosing.clone(), comment)
    }
    // Don't attach comments to empty statement
    // ```javascript
    // for /* comment */ (;;);
    // for (;;a++) /* comment */;
    // ```
    else if comment
        .following_node()
        .is_some_and(|following| JsEmptyStatement::can_cast(following.kind()))
    {
        if let Some(preceding) = comment.preceding_node() {
            CommentPlacement::trailing(preceding.clone(), comment)
        } else {
            CommentPlacement::dangling(comment.enclosing_node().clone(), comment)
        }
    } else {
        CommentPlacement::Default(comment)
    }
}

fn handle_variable_declarator_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    let following = match comment.following_node() {
        Some(following) => following,
        None => return CommentPlacement::Default(comment),
    };

    fn is_complex_value(value: &JsSyntaxNode) -> bool {
        matches!(
            value.kind(),
            JsSyntaxKind::JS_OBJECT_EXPRESSION
                | JsSyntaxKind::JS_ARRAY_EXPRESSION
                | JsSyntaxKind::JS_TEMPLATE_EXPRESSION
                | JsSyntaxKind::TS_OBJECT_TYPE
                | JsSyntaxKind::TS_UNION_TYPE
        )
    }

    let enclosing = comment.enclosing_node();
    match enclosing.kind() {
        JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION | JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION => {
            // Makes all comments preceding objects/arrays/templates or block comments leading comments of these nodes.
            // ```javascript
            // let a = // comment
            // { };
            // ```
            if is_complex_value(following) || !comment.kind().is_line() {
                return CommentPlacement::leading(following.clone(), comment);
            }
        }
        JsSyntaxKind::JS_VARIABLE_DECLARATOR => {
            let variable_declarator = JsVariableDeclarator::unwrap_cast(enclosing.clone());

            match variable_declarator.initializer() {
                // ```javascript
                // let obj2 // Comment
                // = {
                //   key: 'val'
                // }
                // ```
                Some(initializer)
                    if initializer.syntax() == following
                        && initializer
                            .expression()
                            .is_ok_and(|expression| is_complex_value(expression.syntax())) =>
                {
                    return CommentPlacement::leading(initializer.into_syntax(), comment);
                }
                _ => {
                    // fall through
                }
            }
        }
        JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
            let parent_kind = enclosing.parent().kind();

            if matches!(
                parent_kind,
                Some(JsSyntaxKind::JS_VARIABLE_DECLARATOR | JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER)
            ) {
                let not_complex =
                    matches!(parent_kind, Some(JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER))
                        || !is_complex_value(following);

                // Keep trailing comments with the id for variable declarators. Necessary because the value is wrapped
                // inside of an initializer clause.
                // ```javascript
                // let a = // comment
                //      b;
                // ```
                if not_complex
                    && !JsCommentStyle::is_suppression(comment.piece().text())
                    && comment.kind().is_line()
                    && comment.preceding_node().is_none()
                {
                    if let Some(prev_node) = enclosing.prev_sibling() {
                        return CommentPlacement::trailing(prev_node, comment);
                    }
                }
            }
        }
        _ => {
            // fall through
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_parameter_comment(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
    // Make all own line comments leading comments of the parameter
    // ```javascript
    // function a(
    //   b
    //   // comment
    //   = c
    // )
    // ```
    match comment.enclosing_node().kind() {
        JsSyntaxKind::JS_FORMAL_PARAMETER | JsSyntaxKind::TS_PROPERTY_PARAMETER => {
            // Keep decorator comments near the decorator
            // Attach leading parameter comments to the last decorator
            // ```javascript
            // class Foo {
            // 	method(
            // 	//leading own line
            // 	/*leading same line*/ @Decorator /*trailing*/
            // 	//leading own line between
            // 	/*leading same line between*/ @dec //trailing
            // 	/*leading parameter*/
            // 	parameter
            // 	) {}
            // }
            // ```
            if let Some(preceding_node) = comment.preceding_node() {
                if comment.following_node().kind() != Some(JsSyntaxKind::JS_DECORATOR) {
                    return CommentPlacement::trailing(preceding_node.clone(), comment);
                }
            } else if comment.text_position().is_own_line() {
                return CommentPlacement::leading(comment.enclosing_node().clone(), comment);
            }
        }
        JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
            if let Some(parameter) = comment
                .enclosing_node()
                .parent()
                .and_then(JsFormalParameter::cast)
            {
                // Keep end of line comments after the `=` trailing comments of the id
                // ```javascript
                // function a(
                //   b = // test
                //     c
                // )
                // ```
                if comment.text_position().is_end_of_line() && comment.preceding_node().is_none() {
                    if let Ok(binding) = parameter.binding() {
                        return CommentPlacement::trailing(binding.into_syntax(), comment);
                    }
                } else if comment.text_position().is_own_line() {
                    return CommentPlacement::leading(parameter.into_syntax(), comment);
                }
            }
        }
        _ => {
            // fall through
        }
    }

    CommentPlacement::Default(comment)
}

/// Format comments preceding the type parameter name in mapped types on the line above.
///
/// This is achieved by making them dangling comments of the mapped type.
///
/// ```javascript
/// type A = {
///   /* comment */
///   [a in A]: string;
/// }
/// ```
fn handle_mapped_type_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    if !matches!(
        comment.enclosing_node().kind(),
        JsSyntaxKind::TS_MAPPED_TYPE
    ) {
        return CommentPlacement::Default(comment);
    }

    if matches!(
        comment.following_node().kind(),
        Some(
            JsSyntaxKind::TS_TYPE_PARAMETER_NAME
                | JsSyntaxKind::TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE,
        )
    ) {
        return CommentPlacement::dangling(comment.enclosing_node().clone(), comment);
    }

    if matches!(
        comment.preceding_node().kind(),
        Some(JsSyntaxKind::TS_TYPE_PARAMETER_NAME)
    ) {
        if let Some(enclosing) = TsMappedType::cast_ref(comment.enclosing_node()) {
            if let Ok(keys_type) = enclosing.keys_type() {
                return CommentPlacement::trailing(keys_type.into_syntax(), comment);
            }
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_union_type_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    match (comment.enclosing_node().kind(), comment.preceding_node()) {
        (JsSyntaxKind::TS_UNION_TYPE, Some(preceding)) => {
            CommentPlacement::trailing(preceding.clone(), comment)
        }
        _ => CommentPlacement::Default(comment),
    }
}

fn handle_import_export_specifier_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    let enclosing_node = comment.enclosing_node();

    match enclosing_node.kind() {
        // Make end of line or own line comments in the middle of an import specifier leading comments of that specifier
        // ```javascript
        // import { a //comment1
        // //comment2
        // //comment3
        // as b} from "";
        // ```
        JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER
        | JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER
        | JsSyntaxKind::JS_EXPORT_NAMED_SHORTHAND_SPECIFIER
        | JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER => {
            CommentPlacement::leading(enclosing_node.clone(), comment)
        }
        // Make end of line or own line comments in the middle of an import assertion a leading comment of the assertion
        JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY => {
            CommentPlacement::leading(enclosing_node.clone(), comment)
        }

        JsSyntaxKind::JS_EXPORT_AS_CLAUSE => {
            if let Some(parent) = enclosing_node.parent() {
                CommentPlacement::leading(parent, comment)
            } else {
                CommentPlacement::Default(comment)
            }
        }

        JsSyntaxKind::JS_EXPORT_NAMED_FROM_CLAUSE => {
            if let Some(following_token) = comment.following_token() {
                // if we are at the end of a list of import specifiers
                // and encounter a comment
                // ```javascript
                // import {
                //   MULTI,
                //   LINE,
                //   THING,
                //   // some comment here
                // } from 'foo'
                // - then attach it as a trailing comment to `THING`
                if matches!(following_token.kind(), JsSyntaxKind::R_CURLY) {
                    if let Some(preceding) = comment.preceding_node() {
                        return CommentPlacement::trailing(preceding.clone(), comment);
                    }
                }
            }
            CommentPlacement::Default(comment)
        }

        _ => CommentPlacement::Default(comment),
    }
}

/// Attach comments before the `async` keyword or `*` token to the preceding node if it exists
/// to ensure these comments are placed before the `async` keyword or `*` token.
/// ```javascript
/// class Foo {
///    @decorator()
///    // comment
///    async method() {}
/// }
fn handle_class_method_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    let enclosing_node = comment.enclosing_node();
    match enclosing_node.kind() {
        JsSyntaxKind::JS_METHOD_CLASS_MEMBER => {
            if let Some(following_token) = comment.following_token() {
                if matches!(
                    following_token.kind(),
                    JsSyntaxKind::ASYNC_KW | JsSyntaxKind::STAR
                ) {
                    if let Some(preceding) = comment.preceding_node() {
                        return CommentPlacement::trailing(preceding.clone(), comment);
                    }
                }
            }
            CommentPlacement::Default(comment)
        }
        _ => CommentPlacement::Default(comment),
    }
}

fn handle_import_named_clause_comments(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    let enclosing_node = comment.enclosing_node();
    match enclosing_node.kind() {
        JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE => {
            if let Some(import_specifiers) = comment
                .preceding_node()
                .and_then(JsNamedImportSpecifiers::cast_ref)
            {
                let specifier_list = import_specifiers.specifiers();
                // attach comments to the last specifier as trailing comments if comments are at the end of the line
                // ```javascript
                // import { a } from // comment
                // "foo"
                // ```
                if specifier_list.len() != 0
                    && comment.text_position() == CommentTextPosition::EndOfLine
                {
                    if let Some(Ok(last_specifier)) = specifier_list.last() {
                        return CommentPlacement::trailing(last_specifier.into_syntax(), comment);
                    }
                } else {
                    // attach comments to the import specifier as leading comments if comments are placed after the from keyword
                    // ```javascript
                    // import {} from // comment
                    // "foo"
                    // ```
                    let is_after_from_keyword = comment
                        .following_token()
                        .map_or(true, |token| token.kind() != JsSyntaxKind::FROM_KW);
                    if is_after_from_keyword {
                        if let Some(following_node) = comment.following_node() {
                            return CommentPlacement::leading(following_node.clone(), comment);
                        }
                    }
                }
            }
            CommentPlacement::Default(comment)
        }
        _ => CommentPlacement::Default(comment),
    }
}

fn handle_array_expression(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
    if let Some(preceding) = comment.preceding_node().and_then(JsArrayHole::cast_ref) {
        CommentPlacement::leading(preceding.into_syntax(), comment)
    } else {
        CommentPlacement::Default(comment)
    }
}

fn place_leading_statement_comment(
    statement: AnyJsStatement,
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    match statement {
        AnyJsStatement::JsBlockStatement(block) => place_block_statement_comment(block, comment),
        statement => CommentPlacement::leading(statement.into_syntax(), comment),
    }
}

fn place_block_statement_comment(
    block_statement: JsBlockStatement,
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    let first_non_empty = block_statement
        .statements()
        .iter()
        .find(|statement| !matches!(statement, AnyJsStatement::JsEmptyStatement(_)));

    match first_non_empty {
        None => CommentPlacement::dangling(block_statement.into_syntax(), comment),
        Some(statement) => CommentPlacement::leading(statement.into_syntax(), comment),
    }
}

/// Returns `true` if `comment` is a [Closure type comment](https://github.com/google/closure-compiler/wiki/Types-in-the-Closure-Type-System)
/// or [TypeScript type comment](https://www.typescriptlang.org/docs/handbook/jsdoc-supported-types.html#type)
pub(crate) fn is_type_comment(comment: &SyntaxTriviaPieceComments<JsLanguage>) -> bool {
    let text = comment.text();

    // Must be a `/**` comment
    if !text.starts_with("/**") {
        return false;
    }

    text.trim_start_matches("/**")
        .trim_end_matches("*/")
        .split_whitespace()
        .any(|word| {
            match word
                .strip_prefix("@type")
                .or_else(|| word.strip_prefix("@satisfies"))
            {
                Some(after) => after.is_empty() || after.starts_with('{'),
                None => false,
            }
        })
}
