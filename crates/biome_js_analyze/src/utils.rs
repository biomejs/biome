use biome_js_factory::make;
use biome_js_syntax::{
    inner_string_text, AnyJsExpression, AnyJsStatement, JsBinaryExpression, JsLanguage,
    JsModuleItemList, JsStatementList, JsSyntaxNode, T,
};
use biome_rowan::{AstNode, BatchMutation, Direction, WalkEvent};
use std::iter;

pub mod batch;
pub mod case;
pub mod rename;
#[cfg(test)]
pub mod tests;

#[derive(Debug, PartialEq)]
pub(crate) enum EscapeError {
    EscapeAtEndOfString,
    InvalidEscapedChar(char),
}

struct InterpretEscapedString<'a> {
    s: std::str::Chars<'a>,
}

impl<'a> Iterator for InterpretEscapedString<'a> {
    type Item = Result<char, EscapeError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.s.next().map(|c| match c {
            '\\' => match self.s.next() {
                None => Err(EscapeError::EscapeAtEndOfString),
                Some('n') => Ok('\n'),
                Some('\\') => Ok('\\'),
                Some(c) => Err(EscapeError::InvalidEscapedChar(c)),
            },
            c => Ok(c),
        })
    }
}

/// unescape
///
pub(crate) fn escape_string(s: &str) -> Result<String, EscapeError> {
    (InterpretEscapedString { s: s.chars() }).collect()
}

/// Utility function to remove a statement node from a syntax tree, by either
/// removing the node from its parent if said parent is a statement list or
/// module item list, or by replacing the statement node with an empty statement
pub(crate) fn remove_statement<N>(mutation: &mut BatchMutation<JsLanguage>, node: &N) -> Option<()>
where
    N: AstNode<Language = JsLanguage> + Into<AnyJsStatement>,
{
    let parent = node.syntax().parent()?;

    if JsStatementList::can_cast(parent.kind()) || JsModuleItemList::can_cast(parent.kind()) {
        mutation.remove_node(node.clone());
    } else {
        mutation.replace_node(
            node.clone().into(),
            AnyJsStatement::JsEmptyStatement(make::js_empty_statement(make::token(T![;]))),
        );
    }

    Some(())
}

/// Verifies that both nodes are equal by checking their descendants (nodes included) kinds
/// and tokens (same kind and inner token text).
pub(crate) fn is_node_equal(a_node: &JsSyntaxNode, b_node: &JsSyntaxNode) -> bool {
    let a_tree = a_node.preorder_with_tokens(Direction::Next);
    let b_tree = b_node.preorder_with_tokens(Direction::Next);
    for (a_event, b_event) in iter::zip(a_tree, b_tree) {
        let (WalkEvent::Enter(a_child), WalkEvent::Enter(b_child)) = (a_event, b_event) else {
            continue;
        };
        if a_child.kind() != b_child.kind() {
            return false;
        }
        let a_token = a_child.as_token();
        let b_token = b_child.as_token();
        match (a_token, b_token) {
            // both are nodes
            (None, None) => continue,
            // one of them is a node
            (None, Some(_)) | (Some(_), None) => return false,
            // both are tokens
            (Some(a), Some(b)) => {
                if inner_string_text(a) != inner_string_text(b) {
                    return false;
                }
                continue;
            }
        }
    }
    true
}

#[derive(Debug, PartialEq)]
pub(crate) enum VariablePosition {
    Right,
    Left,
}

/// Finds the position of a variable relative to a binary expression.
///
/// This function takes a reference to a JsBinaryExpression and the name of a variable as input.
/// It determines whether the variable appears to the left or right of the binary operator within
/// the expression and returns the result as an `Option<VariablePosition>`.
///
/// Depending on your specific expression and variable placement,
/// the result may vary between Left, Right, or None.
pub(crate) fn find_variable_position(
    binary_expression: &JsBinaryExpression,
    variable: &str,
) -> Option<VariablePosition> {
    let operator_range = binary_expression
        .operator_token()
        .ok()?
        .text_trimmed_range();

    binary_expression
        .syntax()
        .children()
        .filter_map(AnyJsExpression::cast)
        .map(|child| child.omit_parentheses())
        .filter(|child| child.syntax().text_trimmed() == variable)
        .find_map(|child| {
            if child.syntax().text_trimmed_range().end() < operator_range.start() {
                Some(VariablePosition::Left)
            } else if operator_range.end() < child.syntax().text_trimmed_range().start() {
                Some(VariablePosition::Right)
            } else {
                None
            }
        })
}

#[cfg(test)]
mod test {
    use crate::utils::{find_variable_position, VariablePosition};
    use biome_js_parser::{parse, JsParserOptions};
    use biome_js_syntax::{JsBinaryExpression, JsFileSource};
    use biome_rowan::AstNode;

    #[test]
    fn find_variable_position_matches_on_left() {
        let source = "(a) + b";
        let parsed = parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let binary_expression = parsed
            .syntax()
            .descendants()
            .find_map(JsBinaryExpression::cast);

        let variable = "a";
        let position = find_variable_position(&binary_expression.expect("not"), variable);

        assert_eq!(position, Some(VariablePosition::Left));
    }

    #[test]
    fn find_variable_position_matches_on_right() {
        let source = "a + b";
        let parsed = parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let binary_expression = parsed
            .syntax()
            .descendants()
            .find_map(JsBinaryExpression::cast);

        let variable = "b";
        let position = find_variable_position(&binary_expression.expect("not"), variable);

        assert_eq!(position, Some(VariablePosition::Right));
    }

    #[test]
    fn find_variable_position_not_match() {
        let source = "a + b";
        let parsed = parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let binary_expression = parsed
            .syntax()
            .descendants()
            .find_map(JsBinaryExpression::cast);

        let variable = "c";
        let position = find_variable_position(&binary_expression.expect("not"), variable);

        assert_eq!(position, None);
    }
}
