//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(unused_mut)]
use biome_html_syntax::{HtmlSyntaxKind, HtmlSyntaxKind::*, T, *};
use biome_rowan::{
    AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind,
};
#[derive(Debug)]
pub struct HtmlSyntaxFactory;
impl SyntaxFactory for HtmlSyntaxFactory {
    type Kind = HtmlSyntaxKind;
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            ASTRO_BOGUS_FRONTMATTER
            | HTML_BOGUS
            | HTML_BOGUS_ATTRIBUTE
            | HTML_BOGUS_ELEMENT
            | HTML_BOGUS_TEXT_EXPRESSION
            | SVELTE_BOGUS_BLOCK
            | VUE_BOGUS_DIRECTIVE
            | VUE_BOGUS_DIRECTIVE_ARGUMENT => {
                RawSyntaxNode::new(kind, children.into_iter().map(Some))
            }
            ASTRO_CLASS_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![class]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AstroDirectiveValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        ASTRO_CLASS_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(ASTRO_CLASS_DIRECTIVE, children)
            }
            ASTRO_CLIENT_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![client]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AstroDirectiveValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        ASTRO_CLIENT_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(ASTRO_CLIENT_DIRECTIVE, children)
            }
            ASTRO_DEFINE_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![define]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AstroDirectiveValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        ASTRO_DEFINE_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(ASTRO_DEFINE_DIRECTIVE, children)
            }
            ASTRO_DIRECTIVE_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [:]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlAttributeName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlAttributeInitializerClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        ASTRO_DIRECTIVE_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(ASTRO_DIRECTIVE_VALUE, children)
            }
            ASTRO_EMBEDDED_CONTENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == HTML_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        ASTRO_EMBEDDED_CONTENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(ASTRO_EMBEDDED_CONTENT, children)
            }
            ASTRO_FRONTMATTER_ELEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [---]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AstroEmbeddedContent::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [---]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        ASTRO_FRONTMATTER_ELEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(ASTRO_FRONTMATTER_ELEMENT, children)
            }
            ASTRO_IS_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![is]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AstroDirectiveValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        ASTRO_IS_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(ASTRO_IS_DIRECTIVE, children)
            }
            ASTRO_SERVER_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![server]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AstroDirectiveValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        ASTRO_SERVER_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(ASTRO_SERVER_DIRECTIVE, children)
            }
            ASTRO_SET_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![set]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AstroDirectiveValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        ASTRO_SET_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(ASTRO_SET_DIRECTIVE, children)
            }
            HTML_ATTRIBUTE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && HtmlAttributeName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlAttributeInitializerClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_ATTRIBUTE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_ATTRIBUTE, children)
            }
            HTML_ATTRIBUTE_DOUBLE_TEXT_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{{"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!["}}"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_ATTRIBUTE_DOUBLE_TEXT_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_ATTRIBUTE_DOUBLE_TEXT_EXPRESSION, children)
            }
            HTML_ATTRIBUTE_INITIALIZER_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [=]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyHtmlAttributeInitializer::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_ATTRIBUTE_INITIALIZER_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_ATTRIBUTE_INITIALIZER_CLAUSE, children)
            }
            HTML_ATTRIBUTE_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == HTML_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_ATTRIBUTE_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_ATTRIBUTE_NAME, children)
            }
            HTML_ATTRIBUTE_SINGLE_TEXT_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['{']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_ATTRIBUTE_SINGLE_TEXT_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_ATTRIBUTE_SINGLE_TEXT_EXPRESSION, children)
            }
            HTML_CDATA_SECTION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["<![CDATA["]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == HTML_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!["]]>"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_CDATA_SECTION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_CDATA_SECTION, children)
            }
            HTML_CLOSING_ELEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [<]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [/]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyHtmlTagName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [>]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_CLOSING_ELEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_CLOSING_ELEMENT, children)
            }
            HTML_COMPONENT_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == HTML_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_COMPONENT_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_COMPONENT_NAME, children)
            }
            HTML_CONTENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == HTML_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_CONTENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_CONTENT, children)
            }
            HTML_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<8usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [<]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![!]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![doctype]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![html]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == HTML_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == HTML_STRING_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == HTML_STRING_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [>]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_DIRECTIVE, children)
            }
            HTML_DOUBLE_TEXT_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{{"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!["}}"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_DOUBLE_TEXT_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_DOUBLE_TEXT_EXPRESSION, children)
            }
            HTML_ELEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && HtmlOpeningElement::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlElementList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlClosingElement::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_ELEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_ELEMENT, children)
            }
            HTML_EMBEDDED_CONTENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == HTML_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_EMBEDDED_CONTENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_EMBEDDED_CONTENT, children)
            }
            HTML_MEMBER_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnyHtmlComponentObjectName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [.]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTagName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_MEMBER_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_MEMBER_NAME, children)
            }
            HTML_OPENING_ELEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [<]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyHtmlTagName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlAttributeList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [>]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_OPENING_ELEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_OPENING_ELEMENT, children)
            }
            HTML_ROOT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![UNICODE_BOM]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyAstroFrontmatterElement::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlDirective::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlElementList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![EOF]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_ROOT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_ROOT, children)
            }
            HTML_SELF_CLOSING_ELEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [<]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyHtmlTagName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlAttributeList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [/]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [>]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_SELF_CLOSING_ELEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_SELF_CLOSING_ELEMENT, children)
            }
            HTML_SINGLE_TEXT_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['{']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_SINGLE_TEXT_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_SINGLE_TEXT_EXPRESSION, children)
            }
            HTML_SPREAD_ATTRIBUTE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['{']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [...]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_SPREAD_ATTRIBUTE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_SPREAD_ATTRIBUTE, children)
            }
            HTML_STRING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == HTML_STRING_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_STRING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_STRING, children)
            }
            HTML_TAG_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == HTML_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_TAG_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_TAG_NAME, children)
            }
            HTML_TEXT_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == HTML_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        HTML_TEXT_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(HTML_TEXT_EXPRESSION, children)
            }
            SVELTE_ANIMATE_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![animate]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteDirectiveValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_ANIMATE_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_ANIMATE_DIRECTIVE, children)
            }
            SVELTE_ATTACH_ATTRIBUTE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{@"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![attach]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_ATTACH_ATTRIBUTE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_ATTACH_ATTRIBUTE, children)
            }
            SVELTE_AWAIT_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SvelteAwaitOpeningBlock::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteAwaitClausesList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteAwaitClosingBlock::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_AWAIT_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_AWAIT_BLOCK, children)
            }
            SVELTE_AWAIT_CATCH_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{:"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![catch]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlElementList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_AWAIT_CATCH_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_AWAIT_CATCH_BLOCK, children)
            }
            SVELTE_AWAIT_CATCH_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![catch]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_AWAIT_CATCH_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_AWAIT_CATCH_CLAUSE, children)
            }
            SVELTE_AWAIT_CLOSING_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{/"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![await]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_AWAIT_CLOSING_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_AWAIT_CLOSING_BLOCK, children)
            }
            SVELTE_AWAIT_OPENING_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{#"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![await]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteAwaitThenClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteAwaitCatchClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlElementList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_AWAIT_OPENING_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_AWAIT_OPENING_BLOCK, children)
            }
            SVELTE_AWAIT_THEN_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{:"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![then]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlElementList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_AWAIT_THEN_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_AWAIT_THEN_BLOCK, children)
            }
            SVELTE_AWAIT_THEN_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![then]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_AWAIT_THEN_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_AWAIT_THEN_CLAUSE, children)
            }
            SVELTE_BIND_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![bind]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteDirectiveValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_BIND_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_BIND_DIRECTIVE, children)
            }
            SVELTE_CLASS_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![class]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteDirectiveValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_CLASS_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_CLASS_DIRECTIVE, children)
            }
            SVELTE_CONST_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{@"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![const]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_CONST_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_CONST_BLOCK, children)
            }
            SVELTE_CURLY_DESTRUCTURED_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['{']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteBindingAssignmentBindingList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_CURLY_DESTRUCTURED_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_CURLY_DESTRUCTURED_NAME, children)
            }
            SVELTE_DEBUG_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{@"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![debug]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteBindingList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_DEBUG_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_DEBUG_BLOCK, children)
            }
            SVELTE_DIRECTIVE_MODIFIER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [|]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_DIRECTIVE_MODIFIER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_DIRECTIVE_MODIFIER, children)
            }
            SVELTE_DIRECTIVE_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [:]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySvelteBindingProperty::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteDirectiveModifierList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlAttributeInitializerClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_DIRECTIVE_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_DIRECTIVE_VALUE, children)
            }
            SVELTE_EACH_AS_KEYED_ITEM => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![as]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySvelteEachName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteEachIndex::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteEachKey::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_EACH_AS_KEYED_ITEM.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_EACH_AS_KEYED_ITEM, children)
            }
            SVELTE_EACH_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SvelteEachOpeningBlock::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlElementList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteElseClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteEachClosingBlock::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_EACH_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_EACH_BLOCK, children)
            }
            SVELTE_EACH_CLOSING_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{/"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![each]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_EACH_CLOSING_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_EACH_CLOSING_BLOCK, children)
            }
            SVELTE_EACH_INDEX => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [,]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_EACH_INDEX.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_EACH_INDEX, children)
            }
            SVELTE_EACH_KEY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_EACH_KEY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_EACH_KEY, children)
            }
            SVELTE_EACH_KEYED_ITEM => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SvelteEachIndex::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_EACH_KEYED_ITEM.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_EACH_KEYED_ITEM, children)
            }
            SVELTE_EACH_OPENING_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{#"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![each]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySvelteBlockItem::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_EACH_OPENING_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_EACH_OPENING_BLOCK, children)
            }
            SVELTE_ELSE_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{:"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![else]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlElementList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_ELSE_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_ELSE_CLAUSE, children)
            }
            SVELTE_ELSE_IF_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{:"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![else]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![if]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlElementList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_ELSE_IF_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_ELSE_IF_CLAUSE, children)
            }
            SVELTE_HTML_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{@"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![html]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_HTML_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_HTML_BLOCK, children)
            }
            SVELTE_IF_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SvelteIfOpeningBlock::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteElseIfClauseList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteElseClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteIfClosingBlock::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_IF_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_IF_BLOCK, children)
            }
            SVELTE_IF_CLOSING_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{/"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![if]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_IF_CLOSING_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_IF_CLOSING_BLOCK, children)
            }
            SVELTE_IF_OPENING_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{#"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![if]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlElementList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_IF_OPENING_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_IF_OPENING_BLOCK, children)
            }
            SVELTE_IN_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![in]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteDirectiveValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_IN_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_IN_DIRECTIVE, children)
            }
            SVELTE_KEY_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SvelteKeyOpeningBlock::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlElementList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteKeyClosingBlock::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_KEY_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_KEY_BLOCK, children)
            }
            SVELTE_KEY_CLOSING_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{/"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![key]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_KEY_CLOSING_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_KEY_CLOSING_BLOCK, children)
            }
            SVELTE_KEY_OPENING_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{#"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![key]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_KEY_OPENING_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_KEY_OPENING_BLOCK, children)
            }
            SVELTE_LITERAL => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == HTML_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_LITERAL.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_LITERAL, children)
            }
            SVELTE_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == IDENT
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_NAME, children)
            }
            SVELTE_OUT_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![out]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteDirectiveValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_OUT_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_OUT_DIRECTIVE, children)
            }
            SVELTE_RENDER_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{@"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![render]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_RENDER_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_RENDER_BLOCK, children)
            }
            SVELTE_REST_BINDING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [...]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_REST_BINDING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_REST_BINDING, children)
            }
            SVELTE_SNIPPET_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SvelteSnippetOpeningBlock::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteSnippetClosingBlock::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_SNIPPET_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_SNIPPET_BLOCK, children)
            }
            SVELTE_SNIPPET_CLOSING_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{/"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![snippet]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_SNIPPET_CLOSING_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_SNIPPET_CLOSING_BLOCK, children)
            }
            SVELTE_SNIPPET_OPENING_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["{#"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![snippet]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlTextExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlElementList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_SNIPPET_OPENING_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_SNIPPET_OPENING_BLOCK, children)
            }
            SVELTE_SQUARE_DESTRUCTURED_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['[']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteBindingAssignmentBindingList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![']']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_SQUARE_DESTRUCTURED_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_SQUARE_DESTRUCTURED_NAME, children)
            }
            SVELTE_STYLE_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![style]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteDirectiveValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_STYLE_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_STYLE_DIRECTIVE, children)
            }
            SVELTE_TRANSITION_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![transition]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteDirectiveValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_TRANSITION_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_TRANSITION_DIRECTIVE, children)
            }
            SVELTE_USE_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![use]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SvelteDirectiveValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SVELTE_USE_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SVELTE_USE_DIRECTIVE, children)
            }
            VUE_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == IDENT
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && VueDirectiveArgument::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && VueModifierList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlAttributeInitializerClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        VUE_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(VUE_DIRECTIVE, children)
            }
            VUE_DIRECTIVE_ARGUMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [:]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyVueDirectiveArgument::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        VUE_DIRECTIVE_ARGUMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(VUE_DIRECTIVE_ARGUMENT, children)
            }
            VUE_DYNAMIC_ARGUMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['[']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == HTML_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![']']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        VUE_DYNAMIC_ARGUMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(VUE_DYNAMIC_ARGUMENT, children)
            }
            VUE_MODIFIER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [.]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == HTML_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        VUE_MODIFIER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(VUE_MODIFIER, children)
            }
            VUE_STATIC_ARGUMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == HTML_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        VUE_STATIC_ARGUMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(VUE_STATIC_ARGUMENT, children)
            }
            VUE_V_BIND_SHORTHAND_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && VueDirectiveArgument::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && VueModifierList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlAttributeInitializerClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        VUE_V_BIND_SHORTHAND_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(VUE_V_BIND_SHORTHAND_DIRECTIVE, children)
            }
            VUE_V_ON_SHORTHAND_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [@]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyVueDirectiveArgument::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && VueModifierList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlAttributeInitializerClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        VUE_V_ON_SHORTHAND_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(VUE_V_ON_SHORTHAND_DIRECTIVE, children)
            }
            VUE_V_SLOT_SHORTHAND_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [#]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyVueDirectiveArgument::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && VueModifierList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && HtmlAttributeInitializerClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        VUE_V_SLOT_SHORTHAND_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(VUE_V_SLOT_SHORTHAND_DIRECTIVE, children)
            }
            HTML_ATTRIBUTE_LIST => {
                Self::make_node_list_syntax(kind, children, AnyHtmlAttribute::can_cast)
            }
            HTML_ELEMENT_LIST => {
                Self::make_node_list_syntax(kind, children, AnyHtmlElement::can_cast)
            }
            SVELTE_AWAIT_CLAUSES_LIST => {
                Self::make_node_list_syntax(kind, children, AnySvelteAwaitClauses::can_cast)
            }
            SVELTE_BINDING_ASSIGNMENT_BINDING_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnySvelteBindingAssignmentBinding::can_cast,
                T ! [,],
                false,
            ),
            SVELTE_BINDING_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                SvelteName::can_cast,
                T ! [,],
                false,
            ),
            SVELTE_DIRECTIVE_MODIFIER_LIST => {
                Self::make_node_list_syntax(kind, children, SvelteDirectiveModifier::can_cast)
            }
            SVELTE_ELSE_IF_CLAUSE_LIST => {
                Self::make_node_list_syntax(kind, children, SvelteElseIfClause::can_cast)
            }
            VUE_MODIFIER_LIST => Self::make_node_list_syntax(kind, children, VueModifier::can_cast),
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
