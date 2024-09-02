//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use biome_markdown_syntax::{
    MarkdownSyntaxElement as SyntaxElement, MarkdownSyntaxNode as SyntaxNode,
    MarkdownSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn any_value(number_value_list: NumberValueList) -> AnyValue {
    AnyValue::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::ANY_VALUE,
        [Some(SyntaxElement::Node(number_value_list.into_syntax()))],
    ))
}
pub fn number_value(value_token: SyntaxToken) -> NumberValue {
    NumberValue::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::NUMBER_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn root(value: AnyValue, eof_token: SyntaxToken) -> RootBuilder {
    RootBuilder {
        value,
        eof_token,
        bom_token: None,
    }
}
pub struct RootBuilder {
    value: AnyValue,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl RootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> Root {
        Root::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn number_value_list<I>(items: I) -> NumberValueList
where
    I: IntoIterator<Item = NumberValue>,
    I::IntoIter: ExactSizeIterator,
{
    NumberValueList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::NUMBER_VALUE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn bogus<I>(slots: I) -> Bogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    Bogus::unwrap_cast(SyntaxNode::new_detached(MarkdownSyntaxKind::BOGUS, slots))
}
