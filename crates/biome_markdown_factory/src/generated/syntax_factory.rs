//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_markdown_syntax::{MarkdownSyntaxKind, MarkdownSyntaxKind::*, T, *};
use biome_rowan::{
    AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind,
};
#[derive(Debug)]
pub struct MarkdownSyntaxFactory;
impl SyntaxFactory for MarkdownSyntaxFactory {
    type Kind = MarkdownSyntaxKind;
    #[allow(unused_mut)]
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            BOGUS => RawSyntaxNode::new(kind, children.into_iter().map(Some)),
            ANY_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if NumberValueList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        ANY_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(ANY_VALUE, children)
            }
            NUMBER_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == NUMBER_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        NUMBER_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(NUMBER_VALUE, children)
            }
            ROOT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![UNICODE_BOM] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyValue::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![EOF] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(ROOT.to_bogus(), children.into_iter().map(Some));
                }
                slots.into_node(ROOT, children)
            }
            NUMBER_VALUE_LIST => Self::make_node_list_syntax(kind, children, NumberValue::can_cast),
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
