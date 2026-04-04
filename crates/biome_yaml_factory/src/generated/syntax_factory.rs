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
            YAML_BOGUS
            | YAML_BOGUS_BLOCK_HEADER
            | YAML_BOGUS_BLOCK_MAP_ENTRY
            | YAML_BOGUS_BLOCK_NODE
            | YAML_BOGUS_FLOW_NODE => RawSyntaxNode::new(kind, children.into_iter().map(Some)),
            YAML_ALIAS_NODE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == ALIAS_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
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
                if let Some(element) = &current_element
                    && element.kind() == ANCHOR_PROPERTY_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
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
            YAML_BLOCK_CONTENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == BLOCK_CONTENT_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_CONTENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_CONTENT, children)
            }
            YAML_BLOCK_IN_BLOCK_NODE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && YamlPropertyList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyYamlBlockInBlockContent::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_IN_BLOCK_NODE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_IN_BLOCK_NODE, children)
            }
            YAML_BLOCK_KEEP_INDICATOR => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [+]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_KEEP_INDICATOR.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_KEEP_INDICATOR, children)
            }
            YAML_BLOCK_MAP_EXPLICIT_ENTRY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [?]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyYamlBlockNode::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [:]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyYamlBlockNode::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
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
            YAML_BLOCK_MAP_IMPLICIT_ENTRY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnyYamlMappingImplicitKey::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [:]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyYamlBlockNode::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
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
            YAML_BLOCK_MAPPING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == MAPPING_START
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && YamlBlockMapEntryList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == MAPPING_END
                {
                    slots.mark_present();
                    current_element = elements.next();
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
                if let Some(element) = &current_element
                    && element.kind() == SEQUENCE_START
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && YamlBlockSequenceEntryList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == SEQUENCE_END
                {
                    slots.mark_present();
                    current_element = elements.next();
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
                if let Some(element) = &current_element
                    && element.kind() == T ! [-]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyYamlBlockNode::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
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
            YAML_BLOCK_STRIP_INDICATOR => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [-]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_STRIP_INDICATOR.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_STRIP_INDICATOR, children)
            }
            YAML_DIRECTIVE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == DIRECTIVE_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
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
                if let Some(element) = &current_element
                    && element.kind() == T![UNICODE_BOM]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && YamlDirectiveList::can_cast(element.kind())
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
                if let Some(element) = &current_element
                    && AnyYamlBlockNode::can_cast(element.kind())
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
                if let Some(element) = &current_element
                    && element.kind() == DOUBLE_QUOTED_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
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
            YAML_FLOW_IN_BLOCK_NODE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == FLOW_START
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyYamlFlowNode::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == FLOW_END
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_FLOW_IN_BLOCK_NODE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_FLOW_IN_BLOCK_NODE, children)
            }
            YAML_FLOW_JSON_NODE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && YamlPropertyList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyYamlJsonContent::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
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
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [?]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyYamlMappingImplicitKey::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [:]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyYamlFlowNode::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
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
                if let Some(element) = &current_element
                    && AnyYamlMappingImplicitKey::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [:]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyYamlFlowNode::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
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
                if let Some(element) = &current_element
                    && element.kind() == T!['{']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && YamlFlowMapEntryList::can_cast(element.kind())
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
                if let Some(element) = &current_element
                    && element.kind() == T!['[']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && YamlFlowSequenceEntryList::can_cast(element.kind())
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
                if let Some(element) = &current_element
                    && YamlPropertyList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && YamlPlainScalar::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
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
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [>]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && YamlBlockHeaderList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && YamlBlockContent::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
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
            YAML_INDENTATION_INDICATOR => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == INDENTATION_INDICATOR
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_INDENTATION_INDICATOR.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_INDENTATION_INDICATOR, children)
            }
            YAML_LITERAL_SCALAR => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [|]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && YamlBlockHeaderList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && YamlBlockContent::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
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
                if let Some(element) = &current_element
                    && element.kind() == PLAIN_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
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
            YAML_ROOT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && YamlDocumentList::can_cast(element.kind())
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
                        YAML_ROOT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_ROOT, children)
            }
            YAML_SINGLE_QUOTED_SCALAR => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == SINGLE_QUOTED_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
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
            YAML_TAG_PROPERTY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TAG_PROPERTY_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
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
            YAML_BLOCK_HEADER_LIST => {
                Self::make_node_list_syntax(kind, children, AnyYamlBlockHeader::can_cast)
            }
            YAML_BLOCK_MAP_ENTRY_LIST => {
                Self::make_node_list_syntax(kind, children, AnyYamlBlockMapEntry::can_cast)
            }
            YAML_BLOCK_SEQUENCE_ENTRY_LIST => {
                Self::make_node_list_syntax(kind, children, AnyYamlBlockSequenceEntry::can_cast)
            }
            YAML_DIRECTIVE_LIST => {
                Self::make_node_list_syntax(kind, children, YamlDirective::can_cast)
            }
            YAML_DOCUMENT_LIST => {
                Self::make_node_list_syntax(kind, children, AnyYamlDocument::can_cast)
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
            YAML_PROPERTY_LIST => {
                Self::make_node_list_syntax(kind, children, AnyYamlProperty::can_cast)
            }
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
