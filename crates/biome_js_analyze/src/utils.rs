use biome_js_factory::make;
use biome_js_syntax::{
    inner_string_text, AnyJsStatement, JsLanguage, JsModuleItemList, JsStatementList, JsSyntaxNode,
    T,
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
