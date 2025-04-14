//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(unused_mut)]
use biome_rowan::{
    AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind,
};
use biome_yaml_syntax::{T, YamlSyntaxKind, YamlSyntaxKind::*, *};
#[derive(Debug)]
pub struct YamlSyntaxFactory;
impl SyntaxFactory for YamlSyntaxFactory {
    type Kind = YamlSyntaxKind;
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            YAML_BOGUS | YAML_BOGUS_NODE => {
                RawSyntaxNode::new(kind, children.into_iter().map(Some))
            }
            YAML_ALIAS_NODE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == ALIAS_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_ALIAS_NODE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_ALIAS_NODE, children)
            }
            YAML_ANCHOR_PROPERTY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == ANCHOR_PROPERTY_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_ANCHOR_PROPERTY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_ANCHOR_PROPERTY, children)
            }
            YAML_BLOCK_COLLECTION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if YamlPropertyList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyYamlBlockContent::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_COLLECTION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_COLLECTION, children)
            }
            YAML_BLOCK_MAP_EXPLICIT_ENTRY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if YamlBlockMapExplicitKey::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlBlockMapExplicitValue::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_MAP_EXPLICIT_ENTRY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_MAP_EXPLICIT_ENTRY, children)
            }
            YAML_BLOCK_MAP_EXPLICIT_KEY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [?] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlIndentedBlock::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_MAP_EXPLICIT_KEY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_MAP_EXPLICIT_KEY, children)
            }
            YAML_BLOCK_MAP_EXPLICIT_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [:] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlIndentedBlock::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_MAP_EXPLICIT_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_MAP_EXPLICIT_VALUE, children)
            }
            YAML_BLOCK_MAP_IMPLICIT_ENTRY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if AnyYamlBlockMapImplicitKey::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlBlockMapImplicitValue::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_MAP_IMPLICIT_ENTRY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_MAP_IMPLICIT_ENTRY, children)
            }
            YAML_BLOCK_MAP_IMPLICIT_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
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
                        YAML_BLOCK_MAP_IMPLICIT_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_MAP_IMPLICIT_VALUE, children)
            }
            YAML_BLOCK_MAPPING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == INDENT {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlBlockMapEntryList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == DEDENT {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_MAPPING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_MAPPING, children)
            }
            YAML_BLOCK_SEQUENCE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == INDENT {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlBlockSequenceEntryList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == DEDENT {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_SEQUENCE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_SEQUENCE, children)
            }
            YAML_BLOCK_SEQUENCE_ENTRY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [-] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlIndentedBlock::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_SEQUENCE_ENTRY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_SEQUENCE_ENTRY, children)
            }
            YAML_COMPACT_MAPPING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if YamlBlockSequenceEntryList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_COMPACT_MAPPING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_COMPACT_MAPPING, children)
            }
            YAML_COMPACT_SEQUENCE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if YamlBlockSequenceEntryList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_COMPACT_SEQUENCE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_COMPACT_SEQUENCE, children)
            }
            YAML_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == DIRECTIVE_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_DIRECTIVE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_DIRECTIVE, children)
            }
            YAML_DOCUMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![UNICODE_BOM] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlDirectiveList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [---] {
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
                if let Some(element) = &current_element {
                    if element.kind() == T ! [...] {
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
            YAML_DOUBLE_QUOTED_SCALAR => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == DOUBLE_QUOTED_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_DOUBLE_QUOTED_SCALAR.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_DOUBLE_QUOTED_SCALAR, children)
            }
            YAML_FLOW_JSON_NODE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if YamlPropertyList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyYamlJsonContent::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_FLOW_JSON_NODE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_FLOW_JSON_NODE, children)
            }
            YAML_FLOW_MAP_EXPLICIT_ENTRY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [?] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlFlowMapImplicitEntry::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_FLOW_MAP_EXPLICIT_ENTRY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_FLOW_MAP_EXPLICIT_ENTRY, children)
            }
            YAML_FLOW_MAP_IMPLICIT_ENTRY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if AnyYamlFlowMapImplicitKey::can_cast(element.kind()) {
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
                    if AnyYamlFlowNode::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_FLOW_MAP_IMPLICIT_ENTRY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_FLOW_MAP_IMPLICIT_ENTRY, children)
            }
            YAML_FLOW_MAPPING => {
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
                    if YamlFlowMapEntryList::can_cast(element.kind()) {
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
                        YAML_FLOW_MAPPING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_FLOW_MAPPING, children)
            }
            YAML_FLOW_SEQUENCE => {
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
                    if YamlFlowSequenceEntryList::can_cast(element.kind()) {
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
                        YAML_FLOW_SEQUENCE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_FLOW_SEQUENCE, children)
            }
            YAML_FLOW_YAML_NODE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if YamlPropertyList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlPlainScalar::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_FLOW_YAML_NODE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_FLOW_YAML_NODE, children)
            }
            YAML_FOLDED_SCALAR => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == FOLDED_BLOCK_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_FOLDED_SCALAR.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_FOLDED_SCALAR, children)
            }
            YAML_LITERAL_SCALAR => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == LITERAL_BLOCK_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_LITERAL_SCALAR.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_LITERAL_SCALAR, children)
            }
            YAML_PLAIN_SCALAR => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == PLAIN_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_PLAIN_SCALAR.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_PLAIN_SCALAR, children)
            }
            YAML_PROPERTY_LIST => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if AnyYamlProperty::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_PROPERTY_LIST.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_PROPERTY_LIST, children)
            }
            YAML_SINGLE_QUOTED_SCALAR => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == SINGLE_QUOTED_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_SINGLE_QUOTED_SCALAR.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_SINGLE_QUOTED_SCALAR, children)
            }
            YAML_STREAM => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
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
                        YAML_STREAM.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_STREAM, children)
            }
            YAML_TAG_PROPERTY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == TAG_PROPERTY_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_TAG_PROPERTY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_TAG_PROPERTY, children)
            }
            YAML_BLOCK_MAP_ENTRY_LIST => {
                Self::make_node_list_syntax(kind, children, AnyYamlBlockMapEntry::can_cast)
            }
            YAML_BLOCK_SEQUENCE_ENTRY_LIST => {
                Self::make_node_list_syntax(kind, children, YamlBlockSequenceEntry::can_cast)
            }
            YAML_DIRECTIVE_LIST => {
                Self::make_node_list_syntax(kind, children, YamlDirective::can_cast)
            }
            YAML_DOCUMENT_LIST => {
                Self::make_node_list_syntax(kind, children, YamlDocument::can_cast)
            }
            YAML_FLOW_MAP_ENTRY_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnyYamlFlowMapEntry::can_cast,
                T ! [,],
                false,
            ),
            YAML_FLOW_SEQUENCE_ENTRY_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnyYamlFlowSequenceEntry::can_cast,
                T ! [,],
                false,
            ),
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
