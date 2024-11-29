#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use crate::{
    MiniSyntaxElement as SyntaxElement, MiniSyntaxNode as SyntaxNode,
    MiniSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn mini_complex_node(
    complex_token: SyntaxToken,
    r_bracket_token: SyntaxToken,
    list: MiniNodeList,
    l_bracket_token: SyntaxToken,
) -> MiniComplexNode {
    MiniComplexNode::unwrap_cast(SyntaxNode::new_detached(
        MiniSyntaxKind::MINI_COMPLEX_NODE,
        [
            Some(SyntaxElement::Token(complex_token)),
            Some(SyntaxElement::Token(r_bracket_token)),
            Some(SyntaxElement::Node(list.into_syntax())),
            Some(SyntaxElement::Token(l_bracket_token)),
        ],
    ))
}
pub fn mini_root(any_mini_node: AnyMiniNode) -> MiniRoot {
    MiniRoot::unwrap_cast(SyntaxNode::new_detached(
        MiniSyntaxKind::MINI_ROOT,
        [Some(SyntaxElement::Node(any_mini_node.into_syntax()))],
    ))
}
pub fn mini_simple_node(
    simple_token: SyntaxToken,
    r_bracket_token: SyntaxToken,
    ident: MiniString,
    l_bracket_token: SyntaxToken,
) -> MiniSimpleNode {
    MiniSimpleNode::unwrap_cast(SyntaxNode::new_detached(
        MiniSyntaxKind::MINI_SIMPLE_NODE,
        [
            Some(SyntaxElement::Token(simple_token)),
            Some(SyntaxElement::Token(r_bracket_token)),
            Some(SyntaxElement::Node(ident.into_syntax())),
            Some(SyntaxElement::Token(l_bracket_token)),
        ],
    ))
}
pub fn mini_string(value_token: SyntaxToken) -> MiniString {
    MiniString::unwrap_cast(SyntaxNode::new_detached(
        MiniSyntaxKind::MINI_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn mini_node_list<I>(items: I) -> MiniNodeList
where
    I: IntoIterator<Item = AnyMiniNode>,
    I::IntoIter: ExactSizeIterator,
{
    MiniNodeList::unwrap_cast(SyntaxNode::new_detached(
        MiniSyntaxKind::MINI_NODE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn mini_bogus<I>(slots: I) -> MiniBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    MiniBogus::unwrap_cast(SyntaxNode::new_detached(MiniSyntaxKind::MINI_BOGUS, slots))
}
