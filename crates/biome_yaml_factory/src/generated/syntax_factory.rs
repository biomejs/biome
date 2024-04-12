//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_rowan::{
    AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind,
};
use biome_yaml_syntax::{YamlSyntaxKind, YamlSyntaxKind::*, T, *};
#[derive(Debug)]
pub struct YamlSyntaxFactory;
impl SyntaxFactory for YamlSyntaxFactory {
    type Kind = YamlSyntaxKind;
    #[allow(unused_mut)]
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            YAML_BOGUS => RawSyntaxNode::new(kind, children.into_iter().map(Some)),
            YAML_ALIAS => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [*] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![yaml_alias] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_ALIAS.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_ALIAS, children)
            }
            YAML_COMMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [#] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![yaml_comment_text] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_COMMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_COMMENT, children)
            }
            YAML_DOCUMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if AnyYamlNode::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_DOCUMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_DOCUMENT, children)
            }
            YAML_MAPPING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T!['{'] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlMappingEntries::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T!['}'] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_MAPPING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_MAPPING, children)
            }
            YAML_MAPPING_ENTRY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if YamlScalar::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [:] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyYamlNode::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_MAPPING_ENTRY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_MAPPING_ENTRY, children)
            }
            YAML_ROOT => {
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
                    if YamlDocumentList::can_cast(element.kind()) {
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
                    return RawSyntaxNode::new(
                        YAML_ROOT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_ROOT, children)
            }
            YAML_SCALAR => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![SCALAR_VALUE] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_SCALAR.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_SCALAR, children)
            }
            YAML_SEQUENCE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T!['['] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlSequenceElements::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![']'] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_SEQUENCE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_SEQUENCE, children)
            }
            YAML_DOCUMENT_LIST => {
                Self::make_node_list_syntax(kind, children, YamlDocument::can_cast)
            }
            YAML_MAPPING_ENTRIES => Self::make_separated_list_syntax(
                kind,
                children,
                YamlMappingEntry::can_cast,
                T ! [,],
                true,
            ),
            YAML_SEQUENCE_ELEMENTS => Self::make_separated_list_syntax(
                kind,
                children,
                AnyYamlNode::can_cast,
                T ! [,],
                true,
            ),
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
