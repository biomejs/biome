use biome_rowan::{Direction, WalkEvent};

use crate::TailwindSyntaxNode;

/// Verifies that both nodes are equal by checking their descendants (nodes included) kinds
/// and tokens (same kind and inner token text).
pub fn is_node_equal(a_node: &TailwindSyntaxNode, b_node: &TailwindSyntaxNode) -> bool {
    // fast path: check the length of the text first without allocating strings
    if a_node.text_trimmed_range().len() != b_node.text_trimmed_range().len() {
        return false;
    }

    let a_tree = a_node.preorder_with_tokens(Direction::Next);
    let b_tree = b_node.preorder_with_tokens(Direction::Next);
    for (a_event, b_event) in std::iter::zip(a_tree, b_tree) {
        let (a_child, b_child) = match (a_event, b_event) {
            (WalkEvent::Enter(a), WalkEvent::Enter(b)) => (a, b),
            (WalkEvent::Leave(_), WalkEvent::Leave(_)) => continue,
            _ => return false,
        };
        if a_child.kind() != b_child.kind() {
            return false;
        }
        let a_token = a_child.as_token();
        let b_token = b_child.as_token();
        match (a_token, b_token) {
            // both are nodes
            (None, None) => {}
            // one of them is a node
            (None, Some(_)) | (Some(_), None) => return false,
            // both are tokens
            (Some(a), Some(b)) => {
                if a.text_trimmed() != b.text_trimmed() {
                    return false;
                }
            }
        }
    }
    true
}
