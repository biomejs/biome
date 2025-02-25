//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
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
pub fn html_cdata_section(
    cdata_start_token: SyntaxToken,
    content_token: SyntaxToken,
    cdata_end_token: SyntaxToken,
) -> HtmlCdataSection {
    HtmlCdataSection::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_CDATA_SECTION,
        [
            Some(SyntaxElement::Token(cdata_start_token)),
            Some(SyntaxElement::Token(content_token)),
            Some(SyntaxElement::Token(cdata_end_token)),
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
pub fn html_comment(
    comment_start_token: SyntaxToken,
    content_token: SyntaxToken,
    comment_end_token: SyntaxToken,
) -> HtmlComment {
    HtmlComment::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_COMMENT,
        [
            Some(SyntaxElement::Token(comment_start_token)),
            Some(SyntaxElement::Token(content_token)),
            Some(SyntaxElement::Token(comment_end_token)),
        ],
    ))
}
pub fn html_content(value_token: SyntaxToken) -> HtmlContent {
    HtmlContent::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_CONTENT,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn html_directive(
    l_angle_token: SyntaxToken,
    excl_token: SyntaxToken,
    doctype_token: SyntaxToken,
    r_angle_token: SyntaxToken,
) -> HtmlDirectiveBuilder {
    HtmlDirectiveBuilder {
        l_angle_token,
        excl_token,
        doctype_token,
        r_angle_token,
        html_token: None,
        quirk_token: None,
        public_id_token: None,
        system_id_token: None,
    }
}
pub struct HtmlDirectiveBuilder {
    l_angle_token: SyntaxToken,
    excl_token: SyntaxToken,
    doctype_token: SyntaxToken,
    r_angle_token: SyntaxToken,
    html_token: Option<SyntaxToken>,
    quirk_token: Option<SyntaxToken>,
    public_id_token: Option<SyntaxToken>,
    system_id_token: Option<SyntaxToken>,
}
impl HtmlDirectiveBuilder {
    pub fn with_html_token(mut self, html_token: SyntaxToken) -> Self {
        self.html_token = Some(html_token);
        self
    }
    pub fn with_quirk_token(mut self, quirk_token: SyntaxToken) -> Self {
        self.quirk_token = Some(quirk_token);
        self
    }
    pub fn with_public_id_token(mut self, public_id_token: SyntaxToken) -> Self {
        self.public_id_token = Some(public_id_token);
        self
    }
    pub fn with_system_id_token(mut self, system_id_token: SyntaxToken) -> Self {
        self.system_id_token = Some(system_id_token);
        self
    }
    pub fn build(self) -> HtmlDirective {
        HtmlDirective::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::HTML_DIRECTIVE,
            [
                Some(SyntaxElement::Token(self.l_angle_token)),
                Some(SyntaxElement::Token(self.excl_token)),
                Some(SyntaxElement::Token(self.doctype_token)),
                self.html_token.map(|token| SyntaxElement::Token(token)),
                self.quirk_token.map(|token| SyntaxElement::Token(token)),
                self.public_id_token
                    .map(|token| SyntaxElement::Token(token)),
                self.system_id_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.r_angle_token)),
            ],
        ))
    }
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
pub fn html_root(html: HtmlElementList, eof_token: SyntaxToken) -> HtmlRootBuilder {
    HtmlRootBuilder {
        html,
        eof_token,
        bom_token: None,
        directive: None,
    }
}
pub struct HtmlRootBuilder {
    html: HtmlElementList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
    directive: Option<HtmlDirective>,
}
impl HtmlRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn with_directive(mut self, directive: HtmlDirective) -> Self {
        self.directive = Some(directive);
        self
    }
    pub fn build(self) -> HtmlRoot {
        HtmlRoot::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::HTML_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                self.directive
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.html.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn html_self_closing_element(
    l_angle_token: SyntaxToken,
    name: HtmlName,
    attributes: HtmlAttributeList,
    r_angle_token: SyntaxToken,
) -> HtmlSelfClosingElementBuilder {
    HtmlSelfClosingElementBuilder {
        l_angle_token,
        name,
        attributes,
        r_angle_token,
        slash_token: None,
    }
}
pub struct HtmlSelfClosingElementBuilder {
    l_angle_token: SyntaxToken,
    name: HtmlName,
    attributes: HtmlAttributeList,
    r_angle_token: SyntaxToken,
    slash_token: Option<SyntaxToken>,
}
impl HtmlSelfClosingElementBuilder {
    pub fn with_slash_token(mut self, slash_token: SyntaxToken) -> Self {
        self.slash_token = Some(slash_token);
        self
    }
    pub fn build(self) -> HtmlSelfClosingElement {
        HtmlSelfClosingElement::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::HTML_SELF_CLOSING_ELEMENT,
            [
                Some(SyntaxElement::Token(self.l_angle_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.attributes.into_syntax())),
                self.slash_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.r_angle_token)),
            ],
        ))
    }
}
pub fn html_string(value_token: SyntaxToken) -> HtmlString {
    HtmlString::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn html_attribute_list<I>(items: I) -> HtmlAttributeList
where
    I: IntoIterator<Item = AnyHtmlAttribute>,
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
pub fn html_bogus_attribute<I>(slots: I) -> HtmlBogusAttribute
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    HtmlBogusAttribute::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_BOGUS_ATTRIBUTE,
        slots,
    ))
}
pub fn html_bogus_element<I>(slots: I) -> HtmlBogusElement
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    HtmlBogusElement::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_BOGUS_ELEMENT,
        slots,
    ))
}
