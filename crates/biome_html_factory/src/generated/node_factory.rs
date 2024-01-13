//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use biome_html_syntax::{
    HtmlSyntaxElement as SyntaxElement, HtmlSyntaxNode as SyntaxNode,
    HtmlSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn html_attribute(name: HtmlName) -> HtmlAttributeBuilder {
    HtmlAttributeBuilder {
        name,
        initializer: None,
    }
}
pub struct HtmlAttributeBuilder {
    name: HtmlName,
    initializer: Option<HtmlAttributeInitializerClause>,
}
impl HtmlAttributeBuilder {
    pub fn with_initializer(mut self, initializer: HtmlAttributeInitializerClause) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn build(self) -> HtmlAttribute {
        HtmlAttribute::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::HTML_ATTRIBUTE,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn html_attribute_initializer_clause(
    eq_token: SyntaxToken,
    value: HtmlString,
) -> HtmlAttributeInitializerClause {
    HtmlAttributeInitializerClause::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_ATTRIBUTE_INITIALIZER_CLAUSE,
        [
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn html_closing_element(
    l_angle_token: SyntaxToken,
    slash_token: SyntaxToken,
    name: HtmlName,
    r_angle_token: SyntaxToken,
) -> HtmlClosingElement {
    HtmlClosingElement::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_CLOSING_ELEMENT,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn html_directive(
    l_angle_token: SyntaxToken,
    excl_token: SyntaxToken,
    content: HtmlString,
    r_angle_token: SyntaxToken,
) -> HtmlDirective {
    HtmlDirective::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_DIRECTIVE,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Token(excl_token)),
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn html_element(
    opening_element: HtmlOpeningElement,
    children: HtmlElementList,
    closing_element: HtmlClosingElement,
) -> HtmlElement {
    HtmlElement::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_ELEMENT,
        [
            Some(SyntaxElement::Node(opening_element.into_syntax())),
            Some(SyntaxElement::Node(children.into_syntax())),
            Some(SyntaxElement::Node(closing_element.into_syntax())),
        ],
    ))
}
pub fn html_name(value_token: SyntaxToken) -> HtmlName {
    HtmlName::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn html_opening_element(
    l_angle_token: SyntaxToken,
    name: HtmlName,
    attributes: HtmlAttributeList,
    r_angle_token: SyntaxToken,
) -> HtmlOpeningElement {
    HtmlOpeningElement::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_OPENING_ELEMENT,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(attributes.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn html_root(
    directive: HtmlDirective,
    tags: HtmlElementList,
    eof_token: SyntaxToken,
) -> HtmlRootBuilder {
    HtmlRootBuilder {
        directive,
        tags,
        eof_token,
        bom_token: None,
    }
}
pub struct HtmlRootBuilder {
    directive: HtmlDirective,
    tags: HtmlElementList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl HtmlRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> HtmlRoot {
        HtmlRoot::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::HTML_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.directive.into_syntax())),
                Some(SyntaxElement::Node(self.tags.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn html_self_closing_element(
    l_angle_token: SyntaxToken,
    name: HtmlName,
    attributes: HtmlAttributeList,
    slash_token: SyntaxToken,
    r_angle_token: SyntaxToken,
) -> HtmlSelfClosingElement {
    HtmlSelfClosingElement::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_SELF_CLOSING_ELEMENT,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(attributes.into_syntax())),
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn html_string(value_token: SyntaxToken) -> HtmlString {
    HtmlString::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn html_attribute_list<I>(items: I) -> HtmlAttributeList
where
    I: IntoIterator<Item = HtmlAttribute>,
    I::IntoIter: ExactSizeIterator,
{
    HtmlAttributeList::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_ATTRIBUTE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn html_element_list<I>(items: I) -> HtmlElementList
where
    I: IntoIterator<Item = AnyHtmlElement>,
    I::IntoIter: ExactSizeIterator,
{
    HtmlElementList::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_ELEMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn html_bogus<I>(slots: I) -> HtmlBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    HtmlBogus::unwrap_cast(SyntaxNode::new_detached(HtmlSyntaxKind::HTML_BOGUS, slots))
}
